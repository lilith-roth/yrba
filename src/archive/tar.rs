use flate2::{Compression, read::GzEncoder};
use std::path::PathBuf;
use std::{fs::File, path::Path};

pub(crate) fn create_tarball(
    path_to_backup: &Path,
    temporary_folder_config: Option<String>,
) -> Result<PathBuf, std::io::Error> {
    let cache_dir: PathBuf =
        if let Some(temporary_folder_configuration_input) = temporary_folder_config {
            temporary_folder_configuration_input.parse().unwrap()
        } else {
            get_cache_folder()
        };
    let mut backup_archive_temp_file_path = cache_dir.join(
        path_to_backup
            .file_name()
            .expect("Could not generate backup name!"),
    );
    backup_archive_temp_file_path.set_extension("tar.gz");
    log::debug!("Creating archive: {:?}", backup_archive_temp_file_path);
    std::fs::create_dir_all(cache_dir).expect("Could not create temporary folder for archives!");
    let tar_gz = File::create(backup_archive_temp_file_path.clone())
        .expect("Could not generate filepath for temporary file!");
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.follow_symlinks(false);

    let mut final_path_to_backup = path_to_backup;
    let binding = dirs::home_dir().expect("Could not retrieve home directory!");
    let home_dir = binding
        .to_str()
        .expect("Could not convert home directory path object to str!");
    let replace_dir = &path_to_backup
        .as_os_str()
        .to_str()
        .expect("Could not get home directory for tilde path!")
        .replace("~", home_dir);
    if path_to_backup.starts_with("~") {
        final_path_to_backup = Path::new(replace_dir);
    }
    let archivation_result = tar.append_dir_all("", final_path_to_backup.as_os_str());
    if archivation_result.is_err() {
        log::error!(
            "Error adding files to archive: {:?}\nError: {:?}",
            backup_archive_temp_file_path,
            archivation_result.err()
        );
    }
    match tar.finish() {
        Ok(_) => Ok(backup_archive_temp_file_path),
        Err(err) => {
            log::error!(
                "Error finalizing tar archive {:?}\nError: {:?}",
                final_path_to_backup.as_os_str(),
                err
            );
            Err(err)
        }
    }
}

fn get_cache_folder() -> PathBuf {
    let cache_dir_parent = dirs::cache_dir().expect("Could not get temporary directory!");
    cache_dir_parent.join("yrba/")
}
