use crate::config::Config;
use crate::upload::utils::file_name::generate_backup_name;
use anyhow::{Context, anyhow};
use smb::{
    Client, ClientConfig, CreateOptions, File, FileAttributes, FileCreateArgs, Resource, UncPath,
};
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;
use url::Url;

#[allow(clippy::cast_precision_loss)]
pub(crate) async fn upload_smb(file_path: &Path, config: &Config) -> anyhow::Result<()> {
    let client: Client = Client::new(ClientConfig::default());

    // ToDo: Outsource upload into its own function
    // ToDo: Make work for guest authentication
    // ToDo: Add function for removal of old backups
    // ToDo: Write the docs silly!
    // ToDo: Do a cute lil dance

    let remote_url: Url = Url::parse(&config.remote).context("Could not parse remote URL!")?;
    let remote_address = remote_url
        .host()
        .ok_or_else(|| anyhow!("No remote address defined!"))?;
    let share_name = remote_url.path()[1..]
        .split_once('/')
        .ok_or_else(|| anyhow!("Could not get remote share name!"))?
        .0;
    let username = remote_url.username();
    // ToDo: Allow guest user login!
    let Some(password) = &config.smb_password else {
        return Err(anyhow!("No SMB user password defined!"));
    };
    let backup_directory_path = remote_url.path()[1..]
        .split_once('/')
        .ok_or_else(|| anyhow!("Could not get path to store remote backups!"))?
        .1;

    let target_path: UncPath = if backup_directory_path.is_empty() {
        UncPath::from_str(&format!(
            r"\\{remote_address}\{share_name}"
        ))?
    } else {
        UncPath::from_str(&format!(
            r"\\{remote_address}\{share_name}\{backup_directory_path}"
        ))?
    };
    log::debug!("Remote path {target_path}");
    client
        .share_connect(&target_path, username, password.clone())
        .await?;
    let backup_name: &String = &generate_backup_name(file_path)?;
    let file_to_open: UncPath = target_path.with_add_path(backup_name);
    log::debug!("Backup path {file_to_open}");
    log::info!("{file_to_open}");
    let file_open_args: FileCreateArgs =
        FileCreateArgs::make_overwrite(FileAttributes::default(), CreateOptions::default());
    let resource: Resource = client.create_file(&file_to_open, &file_open_args).await?;
    let file: File = resource.unwrap_file();
    let file_locally: std::fs::File =
        std::fs::File::open(file_path).context("Failed to open file to upload!")?;
    let file_size = file_locally
        .metadata()
        .context("Could not get backup file metadata!")?
        .len();
    let mut buf_reader: BufReader<std::fs::File> = BufReader::with_capacity(16384, file_locally);
    let mut buffer = [0; 16384];
    let mut offset: usize = 0;
    loop {
        let bytes_read = buf_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_block(&buffer, offset as u64, None).await?;
        offset += bytes_read;
        let upload_percent = ((offset as f64 / file_size as f64) * 100f64).round();
        log::debug!("Uploaded {upload_percent}% | {offset} - {file_size}");
    }
    file.close().await?;
    client.close().await?;

    Ok(())
}
