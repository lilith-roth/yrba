use crate::config::Config;
use crate::upload::utils::file_name::generate_backup_name;
use anyhow::{Context, anyhow};
use smb::resource::iter_stream::QueryDirectoryStream;
use smb::{
    Client, ClientConfig, CreateDisposition, CreateOptions, Directory, File, FileAccessMask,
    FileAttributes, FileCreateArgs, FileDispositionInformation, FileFullDirectoryInformation,
    Resource, Tree, UncPath,
};
use std::io::{BufReader, Read};
use std::ops::Add;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tokio_stream::StreamExt;
use url::Url;

pub(crate) async fn upload_smb(file_path: &Path, config: &Config) -> anyhow::Result<()> {
    let client: Client = Client::new(ClientConfig::default());
    let remote_url: Url = Url::parse(&config.remote).context("Could not parse remote URL!")?;
    let remote_address = remote_url
        .host()
        .ok_or_else(|| anyhow!("No remote address defined!"))?;
    let share_name = remote_url.path()[1..]
        .split_once('/')
        .ok_or_else(|| anyhow!("Could not get remote share name!"))?
        .0;
    let username = remote_url.username();
    let password: &str = match &config.smb_password {
        None => {
            if username == "guest" {
                ""
            } else {
                return Err(anyhow!("No SMB user password defined!"));
            }
        }
        Some(pw) => pw,
    };

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

    client
        .share_connect(&target_path, username, password.parse()?)
        .await?;

    Box::pin(upload_backup(file_path, &target_path, &client)).await?;
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

async fn upload_backup(
    file_path: &Path,
    target_path: &UncPath,
    client: &Client,
) -> anyhow::Result<()> {
    let backup_name: &String = &generate_backup_name(file_path)?;
    let file_to_open: UncPath = target_path.to_owned().with_add_path(backup_name);
    log::debug!("Backup path {file_to_open}");
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
    if !resource.is_dir() {
        return Err(anyhow!(
            "Expected remote backup directory is for some weird reason not a directory!?"
        ));
    }
    let dir: Directory = resource.unwrap_dir();
    let dir_arc_ref: Arc<Directory> = dir.into();
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
