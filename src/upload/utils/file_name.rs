use std::fs;
use std::fs::DirEntry;
use std::path::Path;

pub fn generate_backup_name(file_path: &Path) -> String {
    let backup_name: String = get_backup_name_stem(file_path);
    format!(
        "{}--{}.tar.gz",
        backup_name,
        chrono::offset::Local::now().format("%Y-%m-%d_%H-%M")
    )
}

pub fn get_backup_name_stem(file_path: &Path) -> String {
    file_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".tar", "")
}

pub fn get_all_backups_older_than_n_newest_backups(
    n: u16,
    backup_stem_name: &str,
    backup_file_path: &Path,
) -> impl Iterator<Item = DirEntry> {
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
    all_files_in_backup_location
        .into_iter()
        .rev()
        .skip(n as usize)
}
