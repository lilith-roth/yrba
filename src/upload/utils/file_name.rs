use anyhow::anyhow;
use std::path::Path;

pub fn generate_backup_name(backup_name: &Path) -> anyhow::Result<String> {
    let backup_name: String = format!(
        "{}--{}.tar.gz",
        backup_name
            .file_name()
            .ok_or_else(|| anyhow!("Could not generate backup file name!"))?
            .to_string_lossy()
            .replace(".tar.gz", ""),
        chrono::offset::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    );
    Ok(backup_name)
}

pub fn get_backup_name_stem(file_path: &Path) -> anyhow::Result<String> {
    let name_stem: String = file_path
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

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_backup_name_stem() {
        let file: NamedTempFile = NamedTempFile::new().unwrap();
        let file_path: &Path = file.path();

        let backup_name_stem: String = get_backup_name_stem(file_path).unwrap();
        assert!(backup_name_stem.eq(&file_path.file_name().unwrap().to_string_lossy()));
    }
}
