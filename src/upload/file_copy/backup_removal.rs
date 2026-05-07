use std::fs;
use std::fs::{DirEntry, Metadata, ReadDir};
use std::path::Path;
use std::time::SystemTime;

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
