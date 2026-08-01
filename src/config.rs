use anyhow::{Context, anyhow};
use std::fs;
use std::path::PathBuf;
use toml::value::Array;

const DEFAULT_CONFIG: &str = include_str!("../config.example.toml");
const DEFAULT_CONFIG_FILE_PATH: &str = "~/.config/yrba/config.toml";
#[cfg(unix)]
const DEFAULT_ROOT_CONFIG_FILE_PATH: &str = "/etc/yrba.toml";

#[derive(serde::Deserialize, Clone)]
pub(crate) struct Config {
    // Remote URL
    pub(crate) remote: String,

    // Amount of backups to keep
    pub(crate) amount_of_backups_to_keep: u16,

    // Path to folders to back up
    pub(crate) folders_to_backup: Array,

    // Path to temporary folder
    pub(crate) temporary_folder: Option<String>,

    pub(crate) sftp: Option<SftpConfig>,

    pub(crate) smb: Option<SmbConfig>,
}

#[derive(serde::Deserialize, Clone)]
pub(crate) struct SftpConfig {
    // SFTP public key path
    pub(crate) sftp_public_key_path: Option<String>,
    pub(crate) sftp_private_key_path: Option<String>,
    pub(crate) sftp_private_key_password: Option<String>,
    // SFTP password
    pub(crate) sftp_password: Option<String>,
    // Enable SFTP compression
    pub(crate) sftp_compression_enabled: Option<bool>,
    // SFTP file buffer size in MiB
    pub(crate) sftp_file_buffer_size: Option<String>,
}

#[derive(serde::Deserialize, Clone)]
pub(crate) struct SmbConfig {
    // SMB password
    pub(crate) smb_password: Option<String>,
}

#[cfg(unix)]
fn get_default_config_path() -> anyhow::Result<PathBuf> {
    let config_file_path: &str = if nix::unistd::geteuid().is_root() {
        DEFAULT_ROOT_CONFIG_FILE_PATH
    } else {
        let home_dir: PathBuf = dirs::home_dir().ok_or_else(|| {
            anyhow!("Could not retrieve user home directory to create config file!")
        })?;
        &DEFAULT_CONFIG_FILE_PATH.replace('~', home_dir.to_string_lossy().as_ref())
    };
    Ok(PathBuf::from(config_file_path))
}

#[cfg(windows)]
fn get_default_config_path() -> anyhow::Result<PathBuf> {
    Ok(PathBuf::from(DEFAULT_CONFIG_FILE_PATH))
}

fn generate_default_config() -> anyhow::Result<PathBuf> {
    let config_file_path: PathBuf = get_default_config_path()?;
    if !fs::exists(&config_file_path)
        .context("Could not check if default config file already exists!")?
    {
        let mut config_file_dir_path: PathBuf = config_file_path.clone();
        config_file_dir_path.pop();
        fs::create_dir_all(config_file_dir_path)?;
        fs::write(&config_file_path, DEFAULT_CONFIG)
            .context("Could not create default config file!")?;
    }
    Ok(config_file_path)
}

pub(crate) fn load_config(config_path: Option<&PathBuf>) -> anyhow::Result<Config> {
    let config_path: PathBuf = match config_path.cloned() {
        None => generate_default_config()?,
        Some(config_path) => config_path,
    };

    let config_path_final: PathBuf = PathBuf::from(&*shellexpand::tilde(
        &config_path
            .to_str()
            .context("Could not convert config path to UTF-8!")?,
    ));

    assert!(
        fs::exists(&config_path_final).is_ok(),
        "Could not find config path!"
    );
    let config_content: String =
        fs::read_to_string(config_path_final).context("Could not read config file!")?;
    let mut config: Config =
        toml::from_str(&config_content).context("Could not parse config file!")?;
    config = check_config(config)?;
    Ok(config)
}

fn check_config(config: Config) -> anyhow::Result<Config> {
    for folder in config.clone().folders_to_backup {
        folder
            .as_str()
            .ok_or_else(|| anyhow!("Could not parse folder to backup: {folder:?}"))?;
    }
    Ok(config)
}
