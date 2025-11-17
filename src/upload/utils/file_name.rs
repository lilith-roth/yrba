use std::arch::aarch64::vzip1_f32;
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
) -> anyhow::Result<impl Iterator<Item = DirEntry>> {
    let mut files_in_backup_location_with_known_creation_date: Vec<DirEntry> = vec![];
    let all_files_in_backup_location = fs::read_dir(backup_file_path)?;
    for file_result in all_files_in_backup_location {
        let file = file_result?;
        if file.metadata().is_err() || file.metadata()?.created().is_err(){
            continue;
        }
        files_in_backup_location_with_known_creation_date.append(&mut vec![file]);
    }
    files_in_backup_location_with_known_creation_date.sort_by_key(|thing| {
        thing
            .metadata()
            .unwrap()
            .created()
            .unwrap()
    });
    let backups_older_n_newest = files_in_backup_location_with_known_creation_date
        .into_iter()
        .rev()
        .skip(n as usize);
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
