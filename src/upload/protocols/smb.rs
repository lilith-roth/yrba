use crate::config::Config;
use crate::upload::utils::file_name::generate_backup_name;
use anyhow::{Context, anyhow};
use smb::resource::iter_stream::QueryDirectoryStream;
use smb::{
    Client, ClientConfig, ConnectionConfig, CreateDisposition, CreateOptions, Directory, File,
    FileAccessMask, FileAttributes, FileCreateArgs, FileDispositionInformation,
    FileFullDirectoryInformation, Resource, Tree, UncPath,
};
use std::io::{BufReader, Read};
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tokio_stream::StreamExt;
use url::{Host, Url};

/// Establishes connection with the remote smb server, uploads backup, and removes old backups.
///
/// # Arguments
/// * `file_path`   - Path to the file to upload
/// * `config`      - Parsed configuration file
pub(crate) async fn upload_smb(file_path: &Path, config: &Config) -> anyhow::Result<()> {
    let remote_url: Url = Url::parse(&config.remote).context("Could not parse remote URL!")?;
    let remote_address = remote_url
        .host()
        .ok_or_else(|| anyhow!("No remote address defined!"))?;
    let remote_port = remote_url.port();
    let share_name = remote_url.path()[1..]
        .split_once('/')
        .ok_or_else(|| anyhow!("Could not get remote share name!"))?
        .0;
    let username = remote_url.username();
    let password: &str = &config.smb_password.clone().unwrap_or_default();
    let backup_directory_path = remote_url.path()[1..]
        .split_once('/')
        .ok_or_else(|| anyhow!("Could not get path to store remote backups!"))?
        .1;

    let target_path: UncPath = if backup_directory_path.is_empty() {
        UncPath::from_str(&format!(r"\\{remote_address}\{share_name}"))?
    } else {
        UncPath::from_str(&format!(
            r"\\{remote_address}\{share_name}\{backup_directory_path}"
        ))?
    };
    log::debug!("Remote path {target_path}");

    let client: Client = Client::new(ClientConfig {
        dfs: false,
        connection: ConnectionConfig {
            port: remote_port,
            timeout: None,
            min_dialect: None,
            max_dialect: None,
            encryption_mode: smb::connection::EncryptionMode::default(),
            allow_unsigned_guest_access: true,
            compression_enabled: true,
            multichannel: smb::connection::MultiChannelConfig::default(),
            client_name: None,
            disable_notifications: false,
            smb2_only_negotiate: false,
            transport: smb::transport::TransportConfig::default(),
            auth_methods: smb::connection::AuthMethodsConfig::default(),
            credits_backlog: None,
            default_transaction_size: None,
        },
        client_guid: smb::Guid::default(),
    });
    client
        .share_connect(&target_path, username, password.parse()?)
        .await?;

    Box::pin(upload_backup(
        file_path,
        &target_path,
        remote_address,
        &client,
    ))
    .await?;
    delete_old_backup(
        config.amount_of_backups_to_keep,
        file_path
            .file_name()
            .ok_or_else(|| anyhow!("Could not retrieve file name of backup archive!"))?
            .to_string_lossy()
            .as_ref(),
        &target_path,
        &client,
    )
    .await?;

    client.close().await?;

    Ok(())
}

/// Generates a file name for the backup that complies with windows' arbitrary arcane file name
/// restrictions.
fn generate_windows_compatible_backup_name(backup_name: &Path) -> anyhow::Result<String> {
    // ':' is a reserved character under windows. blegh
    generate_backup_name(backup_name).map(|name| name.replace(':', "-"))
}

/// Uploads the backup, to be called by `upload_smb` above
///
/// # Arguments
/// * `file_path`   - Path to file to upload
/// * `target_path` - UNC path to backup folder on remote to upload backup to
/// * `client`      - Connected SMB client
async fn upload_backup(
    file_path: &Path,
    target_path: &UncPath,
    remote_address: Host<&str>,
    client: &Client,
) -> anyhow::Result<()> {
    let backup_name: &String = &generate_windows_compatible_backup_name(file_path)?;
    let file_to_open: UncPath = target_path.to_owned().with_add_path(backup_name);
    log::debug!("Backup path {file_to_open}");

    create_remote_path(&file_to_open, remote_address, client).await?;

    let file_open_args: FileCreateArgs =
        FileCreateArgs::make_overwrite(FileAttributes::default(), CreateOptions::default());
    let resource: Resource = client.create_file(&file_to_open, &file_open_args).await?;
    let file: &File = resource
        .as_file()
        .ok_or_else(|| anyhow!("Could not create file on remote server!"))?;
    let file_locally: std::fs::File =
        std::fs::File::open(file_path).context("Failed to open file to upload!")?;
    let file_size: u64 = file_locally
        .metadata()
        .context("Could not get backup file metadata!")?
        .len();
    // ToDo: Make work with a bigger buffer
    let mut buf_reader: BufReader<std::fs::File> = BufReader::with_capacity(16384, file_locally);
    let mut buffer = [0; 16384];
    let mut offset: usize = 0;
    loop {
        let bytes_read: usize = buf_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_block(&buffer, offset as u64, None).await?;
        offset += bytes_read;
        #[allow(clippy::cast_precision_loss)]
        let upload_percent: f64 = ((offset as f64 / file_size as f64) * 100f64).round();
        log::debug!("Uploaded {upload_percent}% | {offset} - {file_size}");
    }
    file.close().await?;

    Ok(())
}

async fn create_remote_path(
    file_to_open: &UncPath,
    remote_address: Host<&str>,
    client: &Client,
) -> anyhow::Result<()> {
    let directory_create_attributes = FileAttributes::default().with_directory(true);
    let directory_create_options: CreateOptions =
        CreateOptions::default().with_directory_file(true);
    let directory_open_args: FileCreateArgs =
        FileCreateArgs::make_create_new(directory_create_attributes, directory_create_options);
    let remote_path_parts = file_to_open
        .path()
        .context("Could not retrieve remote path!")?
        .split('\\');
    let path_parts_amount = remote_path_parts.clone().count();
    let mut checked_paths = String::new();
    for (i, dir_path) in remote_path_parts.enumerate() {
        if i == path_parts_amount - 1 {
            break;
        }
        checked_paths = format!("{checked_paths}\\{dir_path}");
        let dir_create_path = &format!(
            "\\\\{}\\{}{}",
            remote_address,
            file_to_open
                .share()
                .context("Could not retrieve remote share!")?,
            checked_paths
        );
        log::debug!("Creating remote path: {dir_create_path}");
        match client
            .create_file(&UncPath::from_str(dir_create_path)?, &directory_open_args)
            .await
        {
            Ok(_) => (),
            Err(err) => {
                if err.to_string().contains("0xc0000035") {
                    log::debug!(
                        "Object Name Collision (0xc0000035): Directory likely already exists! Ignoring..."
                    );
                } else {
                    return Err(anyhow!(err));
                }
            }
        }
    }
    Ok(())
}

/// Deletes oldest backups on smb remote, and keeps N newest
///
/// # Arguments
/// * `n`           - Amount of backups to keep
/// * `backup_name` - File name of backup archive created in previous steps
/// * `target_path` - UNC path to the backups on remote server
/// * `client`      - Connected SMB client
async fn delete_old_backup(
    n: u16,
    backup_name: &str,
    target_path: &UncPath,
    client: &Client,
) -> anyhow::Result<()> {
    let tree: Arc<Tree> = client.get_tree(target_path).await?;
    let file_access_args: FileAccessMask = FileAccessMask::default().with_generic_read(true);
    let disposition = CreateDisposition::Open;
    let resource: Resource = tree
        .create_directory(
            target_path
                .path()
                .ok_or_else(|| anyhow!("Could not extract target path from remote URL!"))?,
            disposition,
            file_access_args,
        )
        .await?;
    let Resource::Directory(dir) = resource else {
        return Err(anyhow!(
            "Expected remote backup directory is for some weird reason not a directory!?"
        ));
    };
    let dir_arc_ref: Arc<Directory> = Arc::new(dir);
    let dir_info: QueryDirectoryStream<FileFullDirectoryInformation> =
        Directory::query(&dir_arc_ref, "*").await?;
    let mut full_dir_content: Vec<smb::Result<FileFullDirectoryInformation>> = dir_info
        .filter(|x| {
            if let Ok(z) = x {
                z.file_name
                    .to_string()
                    .contains(&backup_name.replace(".tar.gz", ""))
            } else {
                false
            }
        })
        .collect::<Vec<_>>()
        .await;
    dir_arc_ref.close().await?;
    full_dir_content.sort_by_key(|item| item.as_ref().unwrap().creation_time);
    let sorted_dir_content_without_n_newest = full_dir_content.into_iter().rev().skip(n as usize);
    for file_to_delete_raw in sorted_dir_content_without_n_newest
        .into_iter()
        .filter_map(Some)
    {
        let file_to_delete = file_to_delete_raw?;
        let full_file_deletion_path = target_path
            .path()
            .ok_or_else(|| anyhow!("Could not get remote backup directory path!"))?
            .to_string()
            .add("\\")
            .add(&file_to_delete.file_name.to_string());

        let file_to_delete_remote_resource = tree
            .open_existing(
                &full_file_deletion_path,
                FileAccessMask::default().with_generic_all(true),
            )
            .await?;
        let file_to_delete_remote_object = file_to_delete_remote_resource
            .as_file()
            .ok_or_else(|| anyhow!("Backup to be deleted was for some reason not a file!"))?;

        file_to_delete_remote_object
            .set_info(FileDispositionInformation {
                delete_pending: true.into(),
            })
            .await?;
        file_to_delete_remote_object.close().await?;
    }
    Ok(())
}
