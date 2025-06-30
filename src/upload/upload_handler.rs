use super::sftp::upload_sftp;
use crate::config::Config;
use std::path::{Path, PathBuf};
use url::Url;

#[derive(Clone)]
pub(crate) enum UploadMode {
    SFTP,
    NFS,
}

pub(crate) fn get_upload_mode(remote_str: String) -> UploadMode {
    let url = Url::parse(&remote_str).expect("Could not parse remote URL!");
    match url.scheme() {
        "sftp" => UploadMode::SFTP,
        "nfs" => todo!("No NFS support yet!"),
        _ => panic!("Unknown upload mode! {}", url.scheme()),
    }
}

pub(crate) fn upload_file(file_path: PathBuf, upload_mode: UploadMode, config: Config) {
    log::info!("Starting upload...");
    match upload_mode {
        UploadMode::SFTP => upload_sftp(file_path, config),
        UploadMode::NFS => todo!("No NFS support yet!"),
    }
    log::info!("Upload finished!");
}
