use std::{net::TcpStream, path::Path};

use ssh2::Session;
use url::Url;



pub(crate) fn upload_sftp(
    file_path: Box<Path>,
    remote_str: String
) {
    // Parsing remote information from provided remote_str
    let remote_url = Url::parse(&remote_str).expect("Could not parse remote URL!");
    let host = remote_url.host_str().expect("Could not retrieve remote host from URL!");
    let port_raw = remote_url.port();
    let port = match port_raw {
        Some(port) => port,
        None => 21,
    };


    // Connect to SSH
    let tcp = TcpStream::connect(format!("{host}:{port}"));
    let mut session = Session::new().expect("Could not create SSH session!");
}

