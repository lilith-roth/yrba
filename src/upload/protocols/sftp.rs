use anyhow::{Context, anyhow};
use ssh2::{Error, Session, Sftp};
use std::{
    fs::File,
    io::BufReader,
    net::TcpStream,
    path::{Path, PathBuf},
};
use url::Url;

use crate::Config;
use crate::upload::utils::file_name::{generate_backup_name, get_backup_name_stem};

/// Establishes connection with the remote SFTP server, uploads backup, and removes old backups.
///
/// # Arguments
/// * `file_path`   - Path to the file to upload
/// * `config`      - Parsed configuration file
pub(crate) fn upload_sftp(file_path: &Path, config: &Config) -> anyhow::Result<()> {
    // Parsing remote information from provided remote_str
    let remote_url: Url = Url::parse(&config.remote).context("Could not parse remote URL!")?;
    let host: &str = remote_url
        .host_str()
        .expect("Could not retrieve remote host from URL!");
    let port: u16 = remote_url.port().unwrap_or(22);
    let mut username: String = remote_url.username().to_string();
    if username.is_empty() {
        username = whoami::username()
            .context("No username specified and could not retrieve system username!")?;
    }
    let remote_path: &str = remote_url.path();
    let compression_enabled: bool = config.sftp_compression_enabled.unwrap_or(true);
    let upload_buffer_size = usize::try_from(
        size::Size::from_str(config.sftp_file_buffer_size.as_deref().unwrap_or("128 MiB"))
            .context("Could not parse buffer size!")?
            .bytes(),
    )
    .unwrap_or_else(|err| {
        log::error!(
            "Error parsing SFTP file buffer size! Using default of 128 MiB...\nError: {err}"
        );
        128 * 1024 * 1024
    });

    let session: Session = setup_ssh_session(host, port, compression_enabled)?;
    authenticate_ssh(&username, &session, config)?;

    let sftp_session: Sftp = session
        .sftp()
        .context("Could not create SFTP session! Make sure the remote server supports SFTP.")?;

    //sftp_session.mkdir(&Path::new(remote_path), 0600).context("Could not create remote directory for backups!")?;
    create_remote_directory(&remote_path, &sftp_session).context("Could not create remote path!")?;
    upload_backup(
        remote_path,
        &generate_backup_name(file_path)?,
        file_path,
        upload_buffer_size,
        &sftp_session,
    )?;
    delete_old_backups(
        config.amount_of_backups_to_keep,
        &get_backup_name_stem(file_path)?,
        remote_path,
        &sftp_session,
    )?;
    Ok(())
}

/// Deletes oldest backups on SFTP remote, and keeps N newest
///
/// # Arguments
/// * `n`                   - Amount of backups to keep
/// * `backup_name_stem`    - Stem of the backup name, used to identify the backups on remote
/// * `remote_path`         - Path on remote to the directory containing backups
/// * `sftp_session`        - Established SFTP session
fn delete_old_backups(
    n: u16,
    backup_name_stem: &str,
    remote_path: &str,
    sftp_session: &Sftp,
) -> anyhow::Result<()> {
    let mut dir_content = sftp_session.readdir(remote_path).context("Could not read remote backup directory content!")?;
    dir_content.sort_by_key(|x| x.0.file_name().unwrap_or_default().to_os_string());

    let mut skip_counter = n;
    for x in &dir_content {
        let file_name =
            x.0.file_name()
                .ok_or_else(|| anyhow!("Could not retrieve remote file name for deletion!"))?
                .to_string_lossy();

        if !x.1.is_file() || !file_name.contains(backup_name_stem) {
            continue;
        }
        if skip_counter > 0 {
            skip_counter -= 1;
            continue;
        }

        log::debug!("Deleting old backup: {x:?}");
        sftp_session.unlink(&*x.0).context("Could not delete old backup: {x:?}")?;
    }

    Ok(())
}

/// Uploads the backup, to be called by `upload_sftp` above
///
/// # Arguments
/// * `remote_path`         - Path to the backup directory on remote server
/// * `backup_name`         - File name of the backup to upload
/// * `file_path`           - Path to the file to upload on local system
/// * `buffer_size_bytes`   - Size in bytes for the file read buffer
/// * `sftp_session`        - Established SFTP session
fn upload_backup(
    remote_path: &str,
    backup_name: &str,
    file_path: &Path,
    buffer_size_bytes: usize,
    sftp_session: &Sftp,
) -> anyhow::Result<()> {
    // read file
    let file: File = File::open(file_path).context("Failed to open file to upload!")?;
    let mut buf_reader: BufReader<File> = BufReader::with_capacity(buffer_size_bytes, file);

    // Write file to remote
    let remote_file_path: PathBuf = Path::join(Path::new(remote_path), backup_name);
    log::debug!("Uploading to {}", remote_file_path.display());
    let mut remote_file = sftp_session.create(&remote_file_path).context("Could not create file on remote!")?;
    std::io::copy(&mut buf_reader, &mut remote_file).context("Could not write file to remote!")?;
    
    // Closing channel
    remote_file.close().context("Error closing remote file handle!")?;
    Ok(())
}

/// Authenticates the SSH session
///
/// # Arguments
/// * `username`    - Username to log in with
/// * `session`     - SSH session
/// * `config`      - Parsed configuration file
fn authenticate_ssh(username: &str, session: &Session, config: &Config) -> anyhow::Result<()> {
    let settings_config: Config = config.clone();
    let ssh_config_accepted: bool = match settings_config.sftp_public_key_path {
        Some(public_key_path) => {
            let private_key_provided: bool =
                settings_config.sftp_private_key_path.clone().is_some()
                    && settings_config.sftp_private_key_path.clone().unwrap() != "";
            // Making relative paths work, because they didn't for some reason
            let binding: PathBuf =
                dirs::home_dir().context("Could not retrieve user home directory!")?;
            let home_dir: &str = binding
                .to_str()
                .context("Could not convert user home directory path object to str!")?;
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
                return Err(anyhow!(
                    "No SFTP authentication accepted! No password provided."
                ));
            }
            Some(sftp_password) => {
                log::info!("Trying SFTP password authentication...");
                let password_auth_result: Result<(), Error> =
                    session.userauth_password(username, &sftp_password);
                if password_auth_result.is_err() {
                    log::error!("SFTP: Password authentication failed!");
                    return Err(anyhow!("Could not authenticate with SFTP server!"));
                }
            }
        }
    }
    Ok(())
}

/// Creates SSH session
///
/// # Arguments
/// * `host`                    - Remote host IP/domain
/// * `port`                    - Port to connect to
/// * `compression_enabled`     - If connection compression should be enabled
fn setup_ssh_session(host: &str, port: u16, compression_enabled: bool) -> anyhow::Result<Session> {
    // Connect to SSH
    let tcp: TcpStream =
        TcpStream::connect(format!("{host}:{port}")).context("Could not connect to SSH server!")?;
    let mut session: Session = Session::new().context("Could not create SSH session!")?;
    session.set_compress(compression_enabled);
    session.set_tcp_stream(tcp);
    session
        .handshake()
        .context("Could not handshake SSH server!")?;
    Ok(session)
}

fn create_remote_directory(remote_path: &str, sftp_session: &Sftp) -> anyhow::Result<()> {
    remote_path.split("/").for_each(|remote_dir| {sftp_session.opendir(remote_dir); return;});
    Ok(())
}
