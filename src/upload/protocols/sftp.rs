use anyhow::{Context, anyhow};
use ssh2::{ErrorCode, Session, Sftp};
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
    let compression_enabled: bool = config
        .sftp
        .as_ref()
        .and_then(|sftp| sftp.sftp_compression_enabled)
        .unwrap_or(true);
    let upload_buffer_size = usize::try_from(
        size::Size::from_str(
            config
                .sftp
                .as_ref()
                .and_then(|sftp| sftp.sftp_file_buffer_size.as_deref())
                .unwrap_or("128 MiB"),
        )
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

    create_remote_directory(remote_path, &sftp_session).context("Could not create remote path!")?;
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
    let mut dir_content = sftp_session
        .readdir(remote_path)
        .context("Could not read remote backup directory content!")?;
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
        sftp_session
            .unlink(&x.0)
            .context("Could not delete old backup: {x:?}")?;
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
    let mut remote_file = sftp_session
        .create(&remote_file_path)
        .context("Could not create file on remote!")?;
    std::io::copy(&mut buf_reader, &mut remote_file).context("Could not write file to remote!")?;

    // Closing channel
    remote_file
        .close()
        .context("Error closing remote file handle!")?;
    Ok(())
}

fn authenticate_ssh_keys(
    username: &str,
    public_key_path: &str,
    private_key_path: &str,
    private_key_password: Option<&str>,
    session: &Session,
) -> anyhow::Result<()> {
    let pub_path = PathBuf::from(shellexpand::tilde(public_key_path).as_ref());
    let priv_path = PathBuf::from(shellexpand::tilde(private_key_path).as_ref());

    log::info!("Trying SFTP private key authentication...");

    session.userauth_pubkey_file(username, Some(&pub_path), &priv_path, private_key_password)?;

    log::debug!("SFTP private key authentication succeded");

    Ok(())
}

fn authenticate_ssh_password(
    username: &str,
    password: &str,
    session: &Session,
) -> anyhow::Result<()> {
    log::info!("Trying SFTP password authentication...");

    session.userauth_password(username, password)?;

    log::debug!("SFTP password authentication succeded");

    Ok(())
}

/// Authenticates the SSH session
///
/// # Arguments
/// * `username`    - Username to log in with
/// * `session`     - SSH session
/// * `config`      - Parsed configuration file
fn authenticate_ssh(username: &str, session: &Session, config: &Config) -> anyhow::Result<()> {
    let Some(sftp_cfg) = config.sftp.as_ref() else {
        return Err(anyhow!("No SFTP configuration provided!"));
    };

    // Try public key auth
    if let Some(pub_path) = &sftp_cfg.sftp_public_key_path
        && let Some(priv_path) = &sftp_cfg.sftp_private_key_path
    {
        let mut priv_pwd = sftp_cfg.sftp_private_key_password.as_deref();
        if priv_pwd == Some("") {
            priv_pwd = None;
        }
        if let Err(err) = authenticate_ssh_keys(username, pub_path, priv_path, priv_pwd, session) {
            log::error!(
                "SFTP public key authentication failed: {err}\nFalling back to password authentication..."
            );
        } else {
            return Ok(());
        }
    } else if sftp_cfg.sftp_public_key_path.is_some() && sftp_cfg.sftp_private_key_path.is_none() {
        log::error!("SFTP public key configured, but private key is missing!");
    } else if sftp_cfg.sftp_public_key_path.is_none() && sftp_cfg.sftp_private_key_path.is_some() {
        log::error!("SFTP private key configured, but public key is missing!");
    }

    // Try password auth
    if let Some(password) = sftp_cfg.sftp_password.as_deref()
        && !password.is_empty()
    {
        if let Err(err) = authenticate_ssh_password(username, password, session) {
            log::error!("SFTP password authentication failed: {err}")
        } else {
            return Ok(());
        }
    } else {
        log::error!("No password set for SFTP password authentication!");
        return Err(anyhow!("Failed to authenticate SFTP"));
    }

    // No authentication methods worked
    Err(anyhow!("Failed to authenticate SFTP"))
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
    let mut creation_dir: String = String::new();
    for remote_dir in remote_path.split('/') {
        if remote_dir.is_empty() {
            continue;
        }
        creation_dir = creation_dir.clone() + "/" + remote_dir;
        let dir_result = sftp_session.opendir(&creation_dir);
        if dir_result.is_err_and(|err| err.code() == ErrorCode::SFTP(2)) {
            sftp_session
                .mkdir(creation_dir.as_ref(), 0o700)
                .map_err(|err| anyhow!("Could not create remote backup directory: {err:?}"))?;
        }
    }
    Ok(())
}
