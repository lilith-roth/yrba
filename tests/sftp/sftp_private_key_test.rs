use assert_cmd::cargo_bin;
use log::LevelFilter;
use ssh2::Session;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;

const CONFIG: &str = include_str!("./config.sftp.private-key.toml");
const SSH_PRIVATE_KEY: &str = include_str!("./ssh.ed25519");
const SSH_PUBLIC_KEY: &str = include_str!("./ssh.ed25519.pub");
const TEMP_PATH: &str = "/tmp/yrba-test-sftp-private-key/";

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        // Cleanup
        fs::remove_dir_all(TEMP_PATH).unwrap();
    }
}

#[test]
fn test_sftp_private_key_backup() {
    // Setup logger
    let _ = env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .try_init();
    // Setup cleanup
    let _cleanup_setup = Cleanup;

    // Setup test file to back up
    let backup_folder_path = TEMP_PATH.to_owned() + "backup-content/";
    fs::create_dir_all(&backup_folder_path).unwrap();
    let test_backup_file_path = PathBuf::from(backup_folder_path.clone() + "test-doc.txt");
    fs::write(&test_backup_file_path, "TEST").unwrap();
    let test_file_checksum =
        checksums::hash_file(&test_backup_file_path, checksums::Algorithm::CRC32C);
    log::info!("Test file checksum: {test_file_checksum}");

    // Write SSH keys
    let ssh_private_key_path = PathBuf::from(TEMP_PATH.to_owned() + "id_ed25519");
    fs::write(&ssh_private_key_path, SSH_PRIVATE_KEY).unwrap();
    let ssh_public_key_path = PathBuf::from(TEMP_PATH.to_owned() + "id_ed25519.pub");
    fs::write(&ssh_public_key_path, SSH_PUBLIC_KEY).unwrap();

    // Setup output directory
    let output_folder_path = TEMP_PATH.to_owned() + "out/";
    fs::create_dir_all(&output_folder_path).unwrap();

    // Setup config file
    let config_file_path = TEMP_PATH.to_owned() + "config.sftp.private-key.toml";
    let config_path = PathBuf::from(&config_file_path);
    fs::write(&config_path, CONFIG).expect("Could not create default config file!");

    // Run process
    let mut command = Command::new(cargo_bin!("yrba"));
    let exit_code = command
        .args(["-vvvv", "-c", &*config_file_path])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(exit_code.success());

    // Verifying output archive has been created
    // ToDo: Adapt verification for SFTP upload
    // Connect to the local SSH server
    let tcp = TcpStream::connect("127.0.0.1:2222").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    // sess.userauth_agent("testuser").unwrap();
    sess.userauth_pubkey_file(
        "testuser",
        Some(Path::new(&ssh_public_key_path)),
        ssh_private_key_path.as_ref(),
        "".into(),
    )
    .unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel
        .exec(&("basename -a ".to_owned() + &*TEMP_PATH.to_owned() + "/out/* | tail -n 1"))
        .unwrap();
    let mut uploaded_backup_file_name = String::new();
    channel
        .read_to_string(&mut uploaded_backup_file_name)
        .unwrap();
    channel.close().unwrap();
    uploaded_backup_file_name = uploaded_backup_file_name.replace([' ', '\n'], "");

    let remote_backup_download_path =
        &(TEMP_PATH.to_owned() + "out/" + &*uploaded_backup_file_name);
    log::info!("{remote_backup_download_path}");
    let (mut remote_file, stat) = sess
        .scp_recv(Path::new(remote_backup_download_path))
        .unwrap();
    println!("remote file size: {}", stat.size());
    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents).unwrap();

    // Close the channel and wait for the whole content to be tranferred
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();

    let output_file_path = TEMP_PATH.to_owned() + "/out/" + &*uploaded_backup_file_name;
    let file = File::create(&output_file_path);
    file.unwrap().write_all(&contents).unwrap();

    assert!(output_file_path.contains("backup-content"));
    assert!(output_file_path.contains(".tar.gz"));

    // Verifying output archive
    let mut archive = Archive::new(File::open(Path::new(&output_file_path)).unwrap());
    archive
        .unpack(output_folder_path.clone() + "/unpacked")
        .unwrap();
    let unpacked_file = fs::read_dir(output_folder_path + "/unpacked")
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    let unpacked_file_checksum =
        checksums::hash_file(&unpacked_file.path(), checksums::Algorithm::CRC32C);
    log::info!("Unpacked test file checksum: {test_file_checksum}");
    assert_eq!(unpacked_file_checksum, test_file_checksum);
}
