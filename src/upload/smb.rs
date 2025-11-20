use crate::config::Config;
use anyhow::{Context, anyhow};
use binrw::BinWrite;
use smb::Command::Write;
use smb::{
    Client, ClientConfig, CreateOptions, File, FileAccessMask, FileAttributes, FileCreateArgs,
    ReadAt, Resource, UncPath, WriteRequest,
};
use std::io::ErrorKind::WriteZero;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

pub(crate) async fn upload_smb(file_path: &Path, config: &Config) -> anyhow::Result<()> {
    log::info!("START SMB");
    let client: Client = Client::new(ClientConfig::default());

    let target_path: UncPath = UncPath::from_str(r"\\100.68.218.81\slow-access\tmp\yrba-dbg")?;
    client
        .share_connect(&target_path, "dcpacky", "&Wij&^@@rhGH8W".to_string())
        .await?;

    let file_to_open: UncPath = target_path.with_path(
        file_path
            .file_name()
            .ok_or_else(|| anyhow!(""))?
            .try_into()?,
    );
    let file_open_args: FileCreateArgs =
        FileCreateArgs::make_overwrite(FileAttributes::default(), CreateOptions::default());
    let resource: Resource = client.create_file(&file_to_open, &file_open_args).await?;

    let mut file: File = resource.unwrap_file();
    // ToDo: Read from config!
    const BUF_SIZE: usize = 128 * 1024 * 1024;
    let file_locally: std::fs::File =
        std::fs::File::open(file_path).context("Failed to open file to upload!")?;
    let mut buf_reader: BufReader<std::fs::File> = BufReader::with_capacity(BUF_SIZE, file_locally);
    let mut buffer = [0; 1024];
    let mut offset: usize = 0;
    loop {
        let bytes_read = buf_reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        file.write_block(&buffer, offset as u64, None).await?;
        offset += bytes_read;

        println!("Read {} bytes {}", bytes_read, offset);
    }
    file.close().await?;

    Ok(())
}
