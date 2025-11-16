use anyhow::anyhow;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

pub fn generate_backup_name(backup_name: &str) -> String {
    format!(
        "{}--{}.tar.gz",
        backup_name,
        chrono::offset::Local::now().format("%Y-%m-%d_%H-%M")
    )
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

pub fn get_all_backups_older_than_n_newest_backups(
    n: u16,
    backup_stem_name: &str,
    backup_file_path: &Path,
) -> anyhow::Result<impl Iterator<Item = DirEntry>> {
    let mut all_files_in_backup_location: Vec<DirEntry> = fs::read_dir(backup_file_path)
        .expect("Could not read backup directory content!") // ToDo: Improve to not crash
        .filter(|file| {
            file.as_ref()
                .expect("Could not retrieve file reference!")
                .file_name()
                .to_string_lossy()
                .contains(backup_stem_name)
        })
        .map(Result::unwrap)
        .collect();
    all_files_in_backup_location.sort_by_key(|thing| {
        thing
            .metadata()
            .expect("Could not read file metadata!") // ToDo: Improve to not crash
            .created()
            .expect("Could not read file created date!") // ToDo: Improve to not crash
    });
    let backups_older_n_newest = all_files_in_backup_location
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
