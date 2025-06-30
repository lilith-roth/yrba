use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    net::TcpStream,
    path::{Path, PathBuf},
};

use ssh2::Session;
use url::Url;

use crate::Config;

pub(crate) fn upload_sftp(file_path: PathBuf, config: Config) {
    // Parsing remote information from provided remote_str
    let remote_url = Url::parse(&config.remote).expect("Could not parse remote URL!");
    let host = remote_url
        .host_str()
        .expect("Could not retrieve remote host from URL!");
    let port_raw = remote_url.port();
    let port = port_raw.unwrap_or(22);
    let mut username = remote_url.username();
    let system_username = &whoami::username();
    if username.is_empty() {
        username = system_username;
    }
    let remote_path = remote_url.path();

    // Connect to SSH
    let tcp =
        TcpStream::connect(format!("{host}:{port}")).expect("Could not connect to SSH server!");
    let mut session = Session::new().expect("Could not create SSH session!");
    session.set_tcp_stream(tcp);
    session
        .handshake()
        .expect("Could not handshake SSH server!");

    // ToDo: Relative paths don't work for pubkey!
    let ssh_config_accepted = match config.sftp_pubkey_path {
        Some(pubkey_path) => {
            let privkey_provided = if config.sftp_privkey_path.clone().is_some()
                && config.sftp_privkey_path.clone().unwrap() != ""
            {
                true
            } else {
                false
            };
            let success = match privkey_provided {
                true => {
                    let sftp_privkey_password = if (config.sftp_privkey_password.clone().is_some()
                        && config.sftp_privkey_password.clone().unwrap() == "")
                        || config.sftp_privkey_password.clone().is_none()
                    {
                        None
                    } else {
                        Some(config.sftp_privkey_password.unwrap())
                    };
                    let auth_success = session.userauth_pubkey_file(
                        username,
                        Some(Path::new(&pubkey_path)),
                        config.sftp_privkey_path.unwrap().as_ref(),
                        sftp_privkey_password.as_deref(),
                    );
                    auth_success.is_ok()
                }
                false => false,
            };
            success
        }
        None => false,
    };
    if !ssh_config_accepted {
        match config.sftp_password {
            None => {
                log::error!("No SFTP authentication provided!");
                panic!("No SFTP authentication accepted! No password provided.");
            }
            Some(sftp_password) => {
                let password_auth_result = session.userauth_password(username, &sftp_password);
                if password_auth_result.is_err() {
                    log::error!("SFTP: Password authentication failed!");
                    panic!("Could not authenticate with SFTP server!");
                }
            }
        }
    }

    // Create remote path if it does not exist
    let backup_name =
        file_path
            .clone()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".tar", "");
    let mut mkdir_cmd_channel = session.channel_session().unwrap();
    match mkdir_cmd_channel.exec(&format!("mkdir -p {}", remote_path)) {
        Ok(_remote_path_creation_result) => log::info!("Remote path created successfully!"),
        Err(err) => {
            log::error!("Could not create remote path!");
            panic!("Error creating remote path! {err}")
        }
    };

    // read file
    let file_size = fs::metadata(file_path.clone())
        .expect("Could not get temp file metadata!")
        .len() as usize;
    let file = File::open(file_path.clone()).expect("Failed to open file!");
    let mut buf_reader = BufReader::new(file);

    let mut buffer: Vec<u8> = Vec::with_capacity(file_size);
    buf_reader
        .read_to_end(&mut buffer)
        .expect("Failed to read file!");

    // Write file to remote
    let remote_file_name = format!(
        "{}--{}.tar.gz",
        backup_name,
        chrono::offset::Local::now().format("%Y-%m-%d_%H-%M")
    );
    let remote_file_path = Path::join(Path::new(remote_path), remote_file_name);
    let mut remote_file = session
        .scp_send(&remote_file_path, 0o644, file_size as u64, None)
        .expect("Could not start upload!");
    remote_file
        .write_all(buffer.as_mut_slice())
        .expect("Could not write file to remote host!");

    // Delete older backups than N
    // ToDo: move amount of backups to keep to config file!
    // doesn't work for some weird reason, the cmd works in a terminal session though
    let n_backups_to_keep = 5 + 1;
    let mut rm_cmd_channel = session.channel_session().unwrap();
    let delete_cmd =  &format!("ls -A1t {} | tail -n +{} | grep {} | xargs rm", remote_path, n_backups_to_keep, backup_name);
    match rm_cmd_channel.exec(delete_cmd) {
        Ok(_) => (),
        Err(err) => log::error!("Could not delete older backups! {:?}", err)
    };
    let mut s = String::new();
    rm_cmd_channel.read_to_string(&mut s).unwrap();

    // Closing channel
    remote_file.send_eof().expect("Error sending EOF!");
    remote_file.wait_eof().expect("Error waiting for EOF!");
    remote_file.close().expect("Error closing channel!");
    remote_file
        .wait_close()
        .expect("Error waiting for channel closing!");
}
