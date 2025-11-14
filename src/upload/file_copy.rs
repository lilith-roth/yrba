use crate::config::Config;
use crate::upload::utils::file_name::{
    generate_backup_name, get_all_backups_older_than_n_newest_backups, get_backup_name_stem,
};
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

pub fn file_copy_backup(backup_file_path: &Path, config: &Config) {
    let remote_url: Url = Url::parse(&config.remote).expect("Could not parse remote URL!");
    let target_directory: &str = remote_url.path();
    log::debug!(
        "File copy backup: {} -> {}",
        backup_file_path.display(),
        target_directory
    );
    let file_name: String = generate_backup_name(backup_file_path);
    let target_file_path: PathBuf = Path::new(target_directory).join(&file_name);
    let backup_stem_name: String = get_backup_name_stem(backup_file_path);
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
            );
        }
        Err(err) => log::error!(
            "Could not back up {} to {}\nError: {:?}",
            file_name,
            target_file_path.display(),
            err
        ),
    }
}

fn delete_n_old_backups_at_location(
    n: u16,
    backup_stem_name: &str,
    target_backup_location: &Path,
) {
    let backups_older_than_n_newest =
        get_all_backups_older_than_n_newest_backups(n, backup_stem_name, target_backup_location);
    backups_older_than_n_newest.for_each(|backup| fs::remove_file(backup.path()).expect("REASON"));
}
