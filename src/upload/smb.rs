use std::io::BufReader;
use crate::config::Config;
use anyhow::{anyhow, Context};
use smb::{Client, ClientConfig, CreateOptions, File, FileAccessMask, FileAttributes, FileCreateArgs, ReadAt, Resource, UncPath};
use std::path::Path;
use std::str::FromStr;

pub(crate) async fn upload_smb(file_path: &Path, config: &Config) -> anyhow::Result<()> {
    log::info!("START SMB");
    let client: Client = Client::new(ClientConfig::default());

    let target_path: UncPath = UncPath::from_str(r"\\192.168.178.3\slow-access\tmp\yrba-dbg")?;
    client
        .share_connect(&target_path, "dcpacky", "&Wij&^@@rhGH8W".to_string())
        .await?;

    let file_to_open: UncPath = target_path.with_path(
        file_path.file_name().ok_or_else(|| anyhow!(""))?.try_into()?);
    let file_open_args: FileCreateArgs =
        FileCreateArgs::make_overwrite(FileAttributes::default(), CreateOptions::default());
    let resource: Resource = client.create_file(&file_to_open, &file_open_args).await?;

    let file: File = resource.unwrap_file();
    // ToDo: Read from config!
    const BUF_SIZE: usize = 128 * 1024 * 1024;
    let file_locally: std::fs::File = std::fs::File::open(file_path).context("Failed to open file to upload!")?;
    let mut buf_reader: BufReader<std::fs::File> = BufReader::with_capacity(BUF_SIZE, file_locally);


    file.read_at(&mut data, 0).await?;

    // and close
    file.close().await?;

    Ok(())
}
