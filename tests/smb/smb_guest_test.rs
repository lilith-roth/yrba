use assert_cmd::cargo_bin;
use log::LevelFilter;
use smb::resource::iter_stream::QueryDirectoryStream;
use smb::{
    Client, ClientConfig, ConnectionConfig, CreateDisposition, Directory, FileAccessMask,
    FileCreateArgs, FileFullDirectoryInformation, ReadAt, Resource, Tree, UncPath,
};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::sync::Arc;
use tar::Archive;
use tokio_stream::StreamExt;

const CONFIG: &str = include_str!("./config.smb.guest.toml");
const TEMP_PATH: &str = "/tmp/yrba-test-smb-guest/";

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        // Cleanup
        fs::remove_dir_all(TEMP_PATH).unwrap();
    }
}

#[tokio::test]
async fn test_smb_guest_backup() {
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

    // Setup output directory
    let output_folder_path = TEMP_PATH.to_owned() + "out/";
    fs::create_dir_all(&output_folder_path).unwrap();

    // Setup config file
    let config_file_path = TEMP_PATH.to_owned() + "config.smb.guest.toml";
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

    // Connect to SMB within test
    let remote_address = "127.0.0.1";
    let share_name = "content";
    let username = "guest";
    let password: &str = "";
    let backup_directory_path = r"yrba-test-smb-guest\out";
    let target_path: UncPath = UncPath::from_str(&format!(
        r"\\{remote_address}\{share_name}\{backup_directory_path}"
    ))
    .unwrap();

    let client: Client = Client::new(ClientConfig {
        dfs: false,
        connection: ConnectionConfig {
            port: Option::from(4455),
            timeout: None,
            min_dialect: None,
            max_dialect: None,
            encryption_mode: Default::default(),
            allow_unsigned_guest_access: true,
            compression_enabled: false,
            multichannel: Default::default(),
            client_name: None,
            disable_notifications: false,
            smb2_only_negotiate: false,
            transport: Default::default(),
            auth_methods: Default::default(),
            credits_backlog: None,
            default_transaction_size: None,
        },
        client_guid: Default::default(),
    });
    client
        .share_connect(&target_path, username, password.parse().unwrap())
        .await
        .unwrap();

    // Get content of the upload directory
    let tree: Arc<Tree> = client.get_tree(&target_path).await.unwrap();
    let file_access_args: FileAccessMask = FileAccessMask::default().with_generic_read(true);
    let disposition = CreateDisposition::Open;
    let resource: Resource = tree
        .create_directory(target_path.path().unwrap(), disposition, file_access_args)
        .await
        .unwrap();
    let dir = resource.unwrap_dir();
    let dir_arc_ref: Arc<Directory> = Arc::new(dir);
    let dir_info: QueryDirectoryStream<FileFullDirectoryInformation> =
        Directory::query(&dir_arc_ref, "*").await.unwrap();

    let full_dir_content: Vec<smb::Result<FileFullDirectoryInformation>> = dir_info
        .filter(|x| {
            if let Ok(z) = x {
                z.file_name.to_string().contains(&"backup-content")
            } else {
                false
            }
        })
        .collect::<Vec<_>>()
        .await;
    dir_arc_ref.close().await.unwrap();
    let uploaded_file = full_dir_content.first().unwrap().as_ref().unwrap();
    let uploaded_file_name = &uploaded_file.file_name;
    let file_access_args: FileCreateArgs =
        FileCreateArgs::make_open_existing(FileAccessMask::default().with_file_read_data(true));
    let dir_create_path = &format!(
        "\\\\{}\\{}",
        "127.0.0.1\\content\\yrba-test-smb-guest\\out", uploaded_file.file_name
    );
    let uploaded_file_resource = client
        .create_file(
            &UncPath::from_str(dir_create_path).unwrap(),
            &file_access_args,
        )
        .await
        .unwrap()
        .unwrap_file();

    // Download test backup
    let mut buffer = [0; 16384];
    let mut offset: u64 = 0;
    loop {
        let bytes_read: usize = uploaded_file_resource
            .read_at(&mut buffer, offset)
            .await
            .unwrap();
        if bytes_read == 0 {
            break;
        }
        offset += bytes_read as u64;
    }
    uploaded_file_resource.close().await.unwrap();
    client.close().await.unwrap();
    let file = File::create(format!(
        "{}/{}",
        output_folder_path,
        String::from_utf16(&uploaded_file_name).unwrap()
    ))
    .unwrap();
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(&buffer).unwrap();
    buf_writer.flush().unwrap();

    // Verifying output archive has been created
    let file_in_output_dir = fs::read_dir(&output_folder_path)
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    assert!(
        file_in_output_dir
            .file_name()
            .to_string_lossy()
            .contains("backup-content")
    );
    assert!(
        file_in_output_dir
            .file_name()
            .to_string_lossy()
            .contains(".tar.gz")
    );

    // Verifying output archive
    let mut archive = Archive::new(File::open(file_in_output_dir.path()).unwrap());
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
