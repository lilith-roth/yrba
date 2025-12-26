use assert_cmd::cargo_bin;
use log::LevelFilter;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use tar::Archive;

const CONFIG: &str = include_str!("./config.file-copy.toml");
const TEMP_PATH: &str = "/tmp/yrba-test-file-copy/";

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        // Cleanup
        fs::remove_dir_all(TEMP_PATH).unwrap();
    }
}

#[test]
fn test_file_copy_backup() {
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
    let config_file_path = TEMP_PATH.to_owned() + "config.file-copy.toml";
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
