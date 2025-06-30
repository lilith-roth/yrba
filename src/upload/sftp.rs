use std::{fs::{self, File}, io::{BufReader, Read, Write}, net::TcpStream, path::{Path, PathBuf}};

use ssh2::Session;
use url::Url;

use crate::Config;



pub(crate) fn upload_sftp(
    file_path: PathBuf,
    config: Config
) {
    // Parsing remote information from provided remote_str
    let remote_url = Url::parse(&config.remote).expect("Could not parse remote URL!");
    let host = remote_url.host_str().expect("Could not retrieve remote host from URL!");
    let port_raw = remote_url.port();
    let port = port_raw.unwrap_or(22);
    let mut username = remote_url.username();
    let system_username = &whoami::username();
    if username.is_empty() {
        username = system_username;
    }
    let remote_path = remote_url.path();

    // Connect to SSH
    let tcp = TcpStream::connect(format!("{host}:{port}")).expect("Could not connect to SSH server!");
    let mut session = Session::new().expect("Could not create SSH session!");
    session.set_tcp_stream(tcp);
    session.handshake().expect("Could not handshake SSH server!");

    // ToDo: Relative paths don't work for pubkey!
    match config.sftp_pubkey_path {
        Some(pubkey_path) => {
            let sftp_privkey_password = if (config.sftp_privkey_password.clone().is_some() && config.sftp_privkey_password.clone().unwrap() == "") || config.sftp_privkey_password.clone().is_none(){
                None
            } else {
                Some(config.sftp_privkey_password.unwrap())
            };
            session.userauth_pubkey_file(
                username,
                Some(Path::new(&pubkey_path)),
                config.sftp_privkey_path.as_ref(),
                sftp_privkey_password.as_deref()
            ).expect("Could not connect to SFTP server!");
        }
        None => (),
    };
    // let sftp_privkey_path = if config.sftp_privkey_password == "" {
    //     None
    // } else {
    //     Path::new(&config.sftp_privkey_path)
    // };

    // read file
    let file_size = fs::metadata(file_path.clone()).expect("Could not get temp file metadata!").len() as usize;
    let file = File::open(file_path).expect("Failed to open file!");
    let mut buf_reader = BufReader::new(file);

    let mut buffer: Vec<u8> = Vec::with_capacity(file_size);
    buf_reader.read_to_end(&mut buffer).expect("Failed to read file!");

    // Write file to remote
    let mut remote_file = session.scp_send(
        Path::new(remote_path),
        0o644,
        file_size as u64,
        None
    ).expect("Could not start upload!");
    remote_file.write_all(buffer.as_mut_slice()).expect("Could not write file to remote host!");
    // Closing channel
    remote_file.send_eof().expect("Error sending EOF!");
    remote_file.wait_eof().expect("Error waiting for EOF!");
    remote_file.close().expect("Error closing channel!");
    remote_file.wait_close().expect("Error waiting for channel closing!");
}

