use ssh2::{Channel, Error, Session};
use std::{
    fs::{self, File},
    io::{BufReader, Read},
    net::TcpStream,
    path::{Path, PathBuf},
};
use url::Url;

use crate::Config;
use crate::upload::utils::file_name::{generate_backup_name, get_backup_name_stem};

pub(crate) fn upload_sftp(file_path: &Path, config: &Config) {
    // Parsing remote information from provided remote_str
    let remote_url: Url = Url::parse(&config.remote).expect("Could not parse remote URL!");
    let host: &str = remote_url
        .host_str()
        .expect("Could not retrieve remote host from URL!");
    let port: u16 = remote_url.port().unwrap_or(22);
    let mut username: &str = remote_url.username();
    let system_username: &String = &whoami::username();
    if username.is_empty() {
        username = system_username;
    }
    let remote_path: &str = remote_url.path();

    let session: Session = setup_ssh_session(host, port);
    authenticate_ssh(username, &session, config);

    create_remote_directory(remote_path, &session);
    upload_backup(
        remote_path,
        &generate_backup_name(file_path),
        file_path,
        &session,
    );
    delete_old_backups(
        remote_path,
        &get_backup_name_stem(file_path),
        &session,
        config,
    );
}

fn delete_old_backups(
    remote_path: &str,
    backup_name_stem: &str,
    session: &Session,
    config: &Config,
) {
    // Delete older backups than N
    if config.amount_of_backups_to_keep != 0 {
        let mut rm_cmd_channel: Channel = session
            .channel_session()
            .expect("Could not create SSH channel session to delete older backups!"); // ToDo: Improve to not crash
        let delete_cmd: &String = &format!(
            "cd {} && ls -A1t {} | grep {} | tail -n +{} | xargs rm",
            remote_path,
            remote_path,
            backup_name_stem,
            config.amount_of_backups_to_keep + 1
        );
        match rm_cmd_channel.exec(delete_cmd) {
            Ok(()) => log::debug!("Deletion of older backups successful!\nCommand: `{delete_cmd}`"),
            Err(err) => log::error!("Could not delete older backups! {err:?}"),
        }
        let mut s: String = String::new();
        rm_cmd_channel
            .read_to_string(&mut s)
            .expect("Could not read backup deletion command response!"); // ToDo: Improve to not crash
    }
}

/// Uploads file via SSH
fn upload_backup(remote_path: &str, backup_name: &str, file_path: &Path, session: &Session) {
    // ToDo: Read from config!
    const BUF_SIZE: usize = 128 * 1024 * 1024;

    // read file
    let file_size: u64 = fs::metadata(file_path)
        .expect("Could not get temp file metadata!") // ToDo: Improve to not crash
        .len();
    let file: File = File::open(file_path).expect("Failed to open file to upload!");
    let mut buf_reader: BufReader<File> = BufReader::with_capacity(BUF_SIZE, file);

    // Write file to remote
    let remote_file_path: PathBuf = Path::join(Path::new(remote_path), backup_name);
    log::debug!("Uploading to {}", remote_file_path.display());
    let mut remote_file: Channel = session
        .scp_send(&remote_file_path, 0o644, file_size, None)
        .expect("Could not start upload!"); // ToDo: Improve to not crash
    std::io::copy(&mut buf_reader, &mut remote_file).expect("Could not write file to remote host!"); // ToDo: Improve to not crash

    // Closing channel
    remote_file
        .send_eof()
        .expect("Error sending EOF to SSH server!"); // ToDo: Improve to not crash
    remote_file
        .wait_eof()
        .expect("Error waiting for EOF to SSH server!");
    remote_file.close().expect("Error closing SSH channel!"); // ToDo: Improve to not crash
    remote_file
        .wait_close()
        .expect("Error waiting for SSH channel closing!"); // ToDo: Improve to not crash
}

fn create_remote_directory(remote_path: &str, session: &Session) {
    // Create remote path if it does not exist
    let mut mkdir_cmd_channel: Channel = session.channel_session().unwrap();
    let create_dir_cmd: String = format!("mkdir -p {remote_path}");
    match mkdir_cmd_channel.exec(&create_dir_cmd) {
        Ok(_remote_path_creation_result) => {
            log::debug!("Remote path created successfully!\nCommand: {create_dir_cmd:?}");
        }
        Err(err) => {
            log::error!("Could not create remote path!");
            panic!("Error creating remote path!\nError: {err}")
        }
    }
    let mut s: String = String::new();
    mkdir_cmd_channel
        .read_to_string(&mut s)
        .expect("Could not read remote backup directory creation response!");
}

fn authenticate_ssh(username: &str, session: &Session, config: &Config) {
    let settings_config: Config = config.clone();
    let ssh_config_accepted: bool = match settings_config.sftp_public_key_path {
        Some(public_key_path) => {
            let private_key_provided: bool =
                settings_config.sftp_private_key_path.clone().is_some()
                    && settings_config.sftp_private_key_path.clone().unwrap() != "";
            // Making relative paths work, because they didn't for some reason
            let binding: PathBuf =
                dirs::home_dir().expect("Could not retrieve user home directory!");
            let home_dir: &str = binding
                .to_str()
                .expect("Could not convert user home directory path object to str!");
            let sftp_public_key_path: String = public_key_path.as_str().replace('~', home_dir);
            let sftp_private_key_path: String = settings_config
                .sftp_private_key_path
                .unwrap()
                .as_str()
                .replace('~', home_dir);

            let success: bool = if private_key_provided {
                log::debug!("Trying SFTP private key authentication...");
                let sftp_private_key_password =
                    match settings_config.sftp_private_key_password.as_deref() {
                        None | Some("") => None,
                        Some(_) => settings_config.sftp_private_key_password,
                    };
                let auth_success: Result<(), Error> = session.userauth_pubkey_file(
                    username,
                    Some(Path::new(&sftp_public_key_path)),
                    sftp_private_key_path.as_ref(),
                    sftp_private_key_password.as_deref(),
                );
                auth_success.is_ok()
            } else {
                false
            };
            log::debug!("SFTP private key authentication result {success:?}");
            if !success {
                log::warn!("SFTP private key authentication failed!");
            }
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
                let password_auth_result: Result<(), Error> =
                    session.userauth_password(username, &sftp_password);
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
    let tcp: TcpStream =
        TcpStream::connect(format!("{host}:{port}")).expect("Could not connect to SSH server!");
    let mut session: Session = Session::new().expect("Could not create SSH session!");
    session.set_tcp_stream(tcp);
    session
        .handshake()
        .expect("Could not handshake SSH server!");
    session
}
