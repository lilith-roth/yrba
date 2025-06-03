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
    let mut username = remote_url.username();
    let system_username = &whoami::username();
    if username == "" {
        username = system_username;
    }
    let remote_path = remote_url.path();

    // Connect to SSH
    let tcp = TcpStream::connect(format!("{host}:{port}")).expect("Could not connect to SSH server!");
    let mut session = Session::new().expect("Could not create SSH session!");
    session.set_tcp_stream(tcp);
    session.handshake().expect("Could not handshake SSH server!");
    session.userauth_agent(username).expect("Could not authenticate with remote server!");

    // Write file to remote
    let mut remote_file = session.scp_send(
        Path::new(remote_path),
        0o644,
        10,
        None
    );
    todo!("define file & start upload!");
}

