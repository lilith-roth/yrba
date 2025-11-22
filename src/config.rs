use anyhow::Context;
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

    // SFTP Settings
    // SFTP public key path
    pub(crate) sftp_public_key_path: Option<String>,
    pub(crate) sftp_private_key_path: Option<String>,
    pub(crate) sftp_private_key_password: Option<String>,
    // SFTP password
    pub(crate) sftp_password: Option<String>,

    // SMB Settings
    // SMB password
    pub(crate) smb_password: Option<String>,

    // Path to folders to back up
    pub(crate) folders_to_backup: Array,

    // Path to temporary folder
    pub(crate) temporary_folder: Option<String>,
}

#[cfg(unix)]
fn get_default_config_path() -> PathBuf {
    let config_file_path: &str = if nix::unistd::geteuid().is_root() {
        DEFAULT_ROOT_CONFIG_FILE_PATH
    } else {
        DEFAULT_CONFIG_FILE_PATH
    };
    PathBuf::from(config_file_path)
}

#[cfg(windows)]
fn get_default_config_path() -> PathBuf {
    PathBuf::from(DEFAULT_CONFIG_FILE_PATH)
}

fn generate_default_config() -> PathBuf {
    let config_file_path: PathBuf = get_default_config_path();
    if !fs::exists(&config_file_path)
        .expect("Could not check if default config file already exists!")
    {
        fs::write(&config_file_path, DEFAULT_CONFIG)
            .expect("Could not create default config file!");
    }
    config_file_path
}

pub(crate) fn load_config(config_path: Option<&PathBuf>) -> anyhow::Result<Config> {
    let config_path: PathBuf = config_path.cloned().unwrap_or_else(generate_default_config);

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
    config = check_config(config);
    Ok(config)
}

fn check_config(config: Config) -> Config {
    for folder in config.clone().folders_to_backup {
        assert!(
            folder.as_str().is_some(),
            "Could not parse folder to backup: {folder:?}"
        );
    }
    config
}
