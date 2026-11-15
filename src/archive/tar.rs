use anyhow::{Context, anyhow};
use flate2::{Compression, read::GzEncoder};
use std::path::PathBuf;
use std::{fs::File, fs::create_dir_all, fs::remove_file, path::Path};
use tar::Builder;

pub(crate) fn create_tarball(
    path_to_backup: &Path,
    temporary_folder_config: Option<String>,
    compression_level: Compression,
) -> anyhow::Result<PathBuf> {
    let cache_dir: PathBuf =
        if let Some(temporary_folder_configuration_input) = temporary_folder_config {
            temporary_folder_configuration_input
                .parse()
                .context("Could not parse temporary folder input!")?
        } else {
            get_cache_folder()?
        };
    let mut backup_archive_temp_file_path: PathBuf = cache_dir.join(
        path_to_backup
            .file_name()
            .ok_or_else(|| anyhow!("Could not generate backup file name!"))?,
    );
    backup_archive_temp_file_path.set_extension("tar.gz");
    log::debug!(
        "Creating archive: {}",
        backup_archive_temp_file_path.display()
    );
    create_dir_all(cache_dir).context("Could not create temporary folder for archives!")?;
    let tar_gz: File = File::create(backup_archive_temp_file_path.clone())
        .context("Could not generate filepath for temporary file!")?;
    let enc: GzEncoder<File> = GzEncoder::new(tar_gz, compression_level);
    let mut tar: Builder<GzEncoder<File>> = Builder::new(enc);
    tar.follow_symlinks(false);

    let mut final_path_to_backup: &Path = path_to_backup;
    let binding: PathBuf = dirs::home_dir().context("Could not retrieve user home directory!")?;
    let home_dir: &str = binding
        .to_str()
        .context("Could not convert home directory path object to str!")?;
    let replace_dir: &String = &path_to_backup
        .as_os_str()
        .to_str()
        .context("Could not get home directory for input tilde path!")?
        .replace('~', home_dir);
    if path_to_backup.starts_with("~") {
        final_path_to_backup = Path::new(replace_dir);
    }
    let archivation_result: std::io::Result<()> =
        tar.append_dir_all("", final_path_to_backup.as_os_str());
    if let Err(err) = archivation_result {
        log::error!(
            "Error adding files to archive: {}\nError: {err:?}",
            backup_archive_temp_file_path.display()
        );
    }
    match tar.finish() {
        Ok(()) => Ok(backup_archive_temp_file_path),
        Err(err) => {
            log::error!(
                "Error finalizing tar archive {}\nError: {:?}",
                final_path_to_backup.as_os_str().display(),
                err
            );
            log::info!("Trying to delete faulty temporary archive...");
            // Delete temporary archive
            if remove_file(&backup_archive_temp_file_path).is_err() {
                log::error!(
                    "Could not delete temporary archive! Please manually remove: {}",
                    backup_archive_temp_file_path.display()
                );
            }
            Err(anyhow!(err))
        }
    }
}

fn get_cache_folder() -> anyhow::Result<PathBuf> {
    let cache_dir_parent: PathBuf =
        dirs::cache_dir().context("Could not get temporary directory!")?;
    let cache_dir = cache_dir_parent.join("yrba/");
    Ok(cache_dir)
}
