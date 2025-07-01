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
    let port = remote_url.port().unwrap_or(22);
    let mut username = remote_url.username();
    let system_username = &whoami::username();
    if username.is_empty() {
        username = system_username;
    }
    let remote_path = remote_url.path();

    let session = setup_ssh_session(host, port);
    authenticate_ssh(username, session.clone(), config.clone());

    let backup_name = file_path
        .clone()
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".tar", "");

    create_remote_directory(remote_path, session.clone());
    upload_backup(remote_path, backup_name.clone(), file_path, session.clone());
    delete_old_backups(remote_path, backup_name, session, config);
}

fn delete_old_backups(remote_path: &str, backup_name: String, session: Session, config: Config) {
    // Delete older backups than N
    if config.amount_of_backups_to_keep != 0 {
        let mut rm_cmd_channel = session.channel_session().unwrap();
        let delete_cmd = &format!(
            "cd {} && ls -A1t {} | grep {} | tail -n +{} | xargs rm",
            remote_path,
            remote_path,
            backup_name,
            config.amount_of_backups_to_keep + 1
        );
        match rm_cmd_channel.exec(delete_cmd) {
            Ok(_) => log::debug!("Delete cmd for older backups successful!"),
            Err(err) => log::error!("Could not delete older backups! {:?}", err),
        };
        let mut s = String::new();
        rm_cmd_channel.read_to_string(&mut s).unwrap();
    }
}

fn upload_backup(remote_path: &str, backup_name: String, file_path: PathBuf, session: Session) {
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
    log::debug!("Uploading to {:?}", remote_file_path);
    let mut remote_file = session
        .scp_send(&remote_file_path, 0o644, file_size as u64, None)
        .expect("Could not start upload!");
    remote_file
        .write_all(buffer.as_mut_slice())
        .expect("Could not write file to remote host!");

    // Closing channel
    remote_file
        .send_eof()
        .expect("Error sending EOF to SSH server!");
    remote_file
        .wait_eof()
        .expect("Error waiting for EOF to SSH server!");
    remote_file.close().expect("Error closing SSH channel!");
    remote_file
        .wait_close()
        .expect("Error waiting for SSH channel closing!");
}

fn create_remote_directory(remote_path: &str, session: Session) {
    // Create remote path if it does not exist
    let mut mkdir_cmd_channel = session.channel_session().unwrap();
    match mkdir_cmd_channel.exec(&format!("mkdir -p {}", remote_path)) {
        Ok(_remote_path_creation_result) => log::debug!("Remote path created successfully!"),
        Err(err) => {
            log::error!("Could not create remote path!");
            panic!("Error creating remote path! {err}")
        }
    };
    let mut s = String::new();
    mkdir_cmd_channel.read_to_string(&mut s).unwrap();
}

fn authenticate_ssh(username: &str, session: Session, config: Config) {
    let settings_config = config.clone();
    let ssh_config_accepted = match settings_config.sftp_pubkey_path {
        Some(pubkey_path) => {
            let privkey_provided = settings_config.sftp_privkey_path.clone().is_some()
                && settings_config.sftp_privkey_path.clone().unwrap() != "";
            // Making relative paths work, because they didn't for some reason
            let binding = dirs::home_dir().expect("Could not retrieve home directory!");
            let home_dir = binding
                .to_str()
                .expect("Could not convert home directory path object to str!");
            let sftp_pubkey_path = pubkey_path.as_str().replace("~", home_dir);
            let sftp_privkey_path = settings_config
                .sftp_privkey_path
                .unwrap()
                .as_str()
                .replace("~", home_dir);

            let success = match privkey_provided {
                true => {
                    log::info!("Trying SFTP private key authentication...");
                    let sftp_privkey_password =
                        if (settings_config.sftp_privkey_password.clone().is_some()
                            && settings_config.sftp_privkey_password.clone().unwrap() == "")
                            || settings_config.sftp_privkey_password.clone().is_none()
                        {
                            None
                        } else {
                            Some(settings_config.sftp_privkey_password.unwrap())
                        };
                    let auth_success = session.userauth_pubkey_file(
                        username,
                        Some(Path::new(&sftp_pubkey_path)),
                        sftp_privkey_path.as_ref(),
                        sftp_privkey_password.as_deref(),
                    );
                    auth_success.is_ok()
                }
                false => false,
            };
            log::debug!("SFTP private key authentication result {:?}", success);
            success
        }
        None => false,
    };
    if !ssh_config_accepted {
        match settings_config.sftp_password {
            None => {
                log::error!("No SFTP authentication provided!");
                panic!("No SFTP authentication accepted! No password provided.");
            }
            Some(sftp_password) => {
                log::info!("Trying SFTP password authentication...");
                let password_auth_result = session.userauth_password(username, &sftp_password);
                if password_auth_result.is_err() {
                    log::error!("SFTP: Password authentication failed!");
                    panic!("Could not authenticate with SFTP server!");
                }
            }
        }
    }
}

fn setup_ssh_session(host: &str, port: u16) -> Session {
    // Connect to SSH
    let tcp =
        TcpStream::connect(format!("{host}:{port}")).expect("Could not connect to SSH server!");
    let mut session = Session::new().expect("Could not create SSH session!");
    session.set_tcp_stream(tcp);
    session
        .handshake()
        .expect("Could not handshake SSH server!");
    session
}
