use std::path::Path;

use url::Url;



pub(crate) fn upload_sftp(
    file_path: Box<Path>,
    remote_str: String
) {
    let remote_url = Url::parse(&remote_str).expect("Could not parse remote URL!");
    let host = remote_url.host_str()
}

