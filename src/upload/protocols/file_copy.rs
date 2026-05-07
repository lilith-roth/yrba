use crate::config::Config;
use crate::upload::utils::file_name::{generate_backup_name, get_backup_name_stem};
use anyhow::Context;
use std::fs;
use std::fs::{DirEntry, Metadata, ReadDir};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use url::Url;

pub fn file_copy_backup(backup_file_path: &Path, config: &Config) -> anyhow::Result<()> {
    let remote_url: Url =
        Url::parse(&config.remote).context("Can't continue: Could not parse remote URL!")?;
    let target_directory: &str = remote_url.path();
    log::debug!(
        "File copy backup: {} -> {}",
        backup_file_path.display(),
        target_directory
    );
    let backup_stem_name: String = get_backup_name_stem(backup_file_path)?;
    let file_name: String = generate_backup_name(backup_stem_name.as_ref())?;
    let target_file_path: PathBuf = Path::new(target_directory).join(&file_name);
    match fs::copy(backup_file_path, &target_file_path) {
        Ok(_) => {
            log::info!(
                "Successfully copied {} to {}",
                file_name,
                target_file_path.display()
            );
            delete_n_old_backups_at_location(
                config.amount_of_backups_to_keep,
                &backup_stem_name,
                target_directory.as_ref(),
            )?;
        }
        Err(err) => log::error!(
            "Could not back up {} to {}\nError: {:?}",
            file_name,
            target_file_path.display(),
            err
        ),
    }
    Ok(())
}

fn delete_n_old_backups_at_location(
    n: u16,
    backup_stem_name: &str,
    target_backup_location: &Path,
) -> anyhow::Result<()> {
    let backups_older_than_n_newest =
        get_all_backups_older_than_n_newest_backups(n, backup_stem_name, target_backup_location);
    backups_older_than_n_newest?.for_each(|backup| {
        fs::remove_file(backup.path()).unwrap_or_else(|err| {
            log::warn!(
                "Could not delete old backup at {}!\nError: {err}",
                backup.path().display()
            );
        });
    });
    Ok(())
}

/// Retrieves all backups older than the n newest.
///
/// Used to delete older unnecessary backups.
/// In case of file errors, like no metadata etc. just ignore the file, we don't want to
/// accidentally delete anything that might still be important.
pub fn get_all_backups_older_than_n_newest_backups(
    n: u16,
    backup_stem_name: &str,
    backup_file_path: &Path,
) -> anyhow::Result<impl Iterator<Item = DirEntry>> {
    let mut files_in_backup_location_with_known_creation_date: Vec<(SystemTime, DirEntry)> = vec![];
    let all_files_in_backup_location: ReadDir = fs::read_dir(backup_file_path)?;
    for file in all_files_in_backup_location.filter_map(Result::ok) {
        if !file
            .file_name()
            .to_string_lossy()
            .contains(backup_stem_name)
        {
            continue;
        }
        let Ok(Ok(created_date)) = file.metadata().as_ref().map(Metadata::created) else {
            continue;
        };
        files_in_backup_location_with_known_creation_date.append(&mut vec![(created_date, file)]);
    }
    files_in_backup_location_with_known_creation_date.sort_by_key(|(created, _)| *created);
    let backups_older_n_newest = files_in_backup_location_with_known_creation_date
        .into_iter()
        .rev()
        .skip(n as usize)
        .map(|(_, file)| file);
    Ok(backups_older_n_newest)
}
