use super::sftp::upload_sftp;
use crate::config::Config;
use crate::upload::file_copy::file_copy_backup;
use crate::upload::smb::upload_smb;
use anyhow::anyhow;
use std::path::Path;
use url::Url;

#[derive(Clone)]
pub(crate) enum UploadMode {
    File,
    Smb,
    Sftp,
}

pub(crate) fn get_upload_mode(remote_str: &str) -> anyhow::Result<UploadMode> {
    let url: Url = Url::parse(remote_str).expect("Could not parse remote URL!");
    let upload_mode = match url.scheme() {
        "file" => UploadMode::File,
        "sftp" => UploadMode::Sftp,
        "smb" => UploadMode::Smb,
        _ => return Err(anyhow!("Unknown upload mode: {}", url.scheme())),
    };
    Ok(upload_mode)
}

pub(crate) async fn upload_file(
    file_path: &Path,
    upload_mode: &UploadMode,
    config: &Config,
) -> anyhow::Result<()> {
    log::info!("Starting upload...");
    match upload_mode {
        UploadMode::File => file_copy_backup(file_path, config)?,
        UploadMode::Smb => Box::pin(upload_smb(file_path, config)).await?,
        UploadMode::Sftp => upload_sftp(file_path, config)?,
    }
    log::info!("Upload finished!");
    Ok(())
}
