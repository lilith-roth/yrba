use anyhow::anyhow;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

pub fn generate_backup_name(backup_name: &Path) -> anyhow::Result<String> {
    let backup_name = format!(
        "{}--{}.tar.gz",
        backup_name.file_name().ok_or_else(|| anyhow!("Could not generate backup file name!"))?.to_string_lossy(),
        chrono::offset::Local::now().format("%Y-%m-%d_%H-%M")
    );
    Ok(backup_name)
}

pub fn get_backup_name_stem(file_path: &Path) -> anyhow::Result<String> {
    let name_stem = file_path
        .file_stem()
        .ok_or_else(|| anyhow!("Could not retrieve file stem of {}!", file_path.display()))?
        .to_str()
        .ok_or_else(|| {
            anyhow!(
                "File stem contain invalid UTF-8 characters! {}",
                file_path.display()
            )
        })?
        .replace(".tar", "");
    Ok(name_stem)
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
) -> anyhow::Result<Vec<DirEntry>> {
    // ToDo: Convert to for loop to improve error handling
    //          -> ToDo: Make vector tuple of <DirEntry, CreatedDate>, if created date can not be retrieved, don't add it to vector
    let mut backups_older_n_newest: Vec<DirEntry> = vec![];
    let mut all_files_in_backup_location = fs::read_dir(backup_file_path)?;
    let amount_files_in_backup_location = all_files_in_backup_location.by_ref().count();
    for (i, file_result) in all_files_in_backup_location.enumerate() {
        let mut file = file_result?;
        if file.file_name().to_string_lossy().contains(backup_stem_name) {
            if file.metadata().is_err() || file.metadata()?.created().is_err(){
                continue;
            }
            if i > amount_files_in_backup_location - n as usize {
                break;
            }

            backups_older_n_newest.append(&mut vec![file]);
        }
    }
    //     .expect("Could not read backup directory content!") // ToDo: Improve to not crash
    //     .filter(|file| {
    //         file.as_ref()
    //             .expect("Could not retrieve file reference!")
    //             .file_name()
    //             .to_string_lossy()
    //             .contains(backup_stem_name)
    //     })
    //     .map(Result::unwrap)
    //     .collect();
    // all_files_in_backup_location.sort_by_key(|thing| {
    //     thing
    //         .metadata()
    //         .expect("Could not read file metadata!") // ToDo: Improve to not crash
    //         .created()
    //         .expect("Could not read file created date!") // ToDo: Improve to not crash
    // });
    // let backups_older_n_newest = all_files_in_backup_location
    //     .into_iter()
    //     .rev()
    //     .skip(n as usize);
    Ok(backups_older_n_newest)
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_backup_name_stem() {
        let file = NamedTempFile::new().unwrap();
        let file_path = file.path();

        let backup_name_stem = get_backup_name_stem(file_path).unwrap();
        assert!(backup_name_stem.eq(&file_path.file_name().unwrap().to_string_lossy()));
    }
}
