use crate::config::Config;
use crate::upload::utils::file_name::generate_backup_name;
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
use url::Url;

pub(crate) async fn upload_smb(file_path: &Path, config: &Config) -> anyhow::Result<()> {
    log::info!("START SMB");
    let client: Client = Client::new(ClientConfig::default());

    let target_path: UncPath = UncPath::from_str(r"\\100.68.218.81\slow-access")?;
    client
        .share_connect(&target_path, "dcpacky", "&Wij&^@@rhGH8W".to_string())
        .await?;
    //\tmp\yrba-dbg
    let remote_url: Url = Url::parse(&config.remote).context("Could not parse remote URL!")?;
    let remote_path: &str = &remote_url.path()[1..remote_url.path().len()];
    let backup_name = &generate_backup_name(file_path)?;
    let remote_full_path = &format!("{remote_path}/{backup_name}");
    log::info!("{}", remote_full_path);
    let file_to_open: UncPath = target_path.with_path(remote_full_path);
    let file_open_args: FileCreateArgs =
        FileCreateArgs::make_overwrite(FileAttributes::default(), CreateOptions::default());
    let resource: Resource = client.create_file(&file_to_open, &file_open_args).await?;

    let mut file: File = resource.unwrap_file();
    let file_locally: std::fs::File =
        std::fs::File::open(file_path).context("Failed to open file to upload!")?;
    let mut buf_reader: BufReader<std::fs::File> = BufReader::with_capacity(BUF_SIZE, file_locally);
    // ToDo: Read from config!
    const BUF_SIZE: usize = 128 * 1024 * 1024;
    let mut buffer = vec![0; BUF_SIZE];
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
