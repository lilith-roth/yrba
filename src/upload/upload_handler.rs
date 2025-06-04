use std::path::{Path, PathBuf};
use url::Url;

use super::sftp::upload_sftp;

#[derive(Clone)]
pub(crate) enum UploadMode {
    SFTP,
    NFS
}



pub(crate) fn get_upload_mode(
    remote_str: String
) -> UploadMode {
    let url = Url::parse(&remote_str).expect("Could not parse remote URL!");
    match url.scheme() {
        "sftp" => UploadMode::SFTP,
        "nfs" => todo!("No NFS support yet!"),
        _ => panic!("Unknown upload mode! {}", url.scheme())
    }
}

pub(crate) fn upload_file(
    file_path: PathBuf,
    remote_str: String,
    upload_mode: UploadMode
) {
    log::info!("Starting upload...");
    match upload_mode {
        UploadMode::SFTP => upload_sftp(file_path, remote_str),
        UploadMode::NFS => todo!("No NFS support yet!"),
    }
    log::info!("Upload finished!");
}

