use std::fs;
use toml::value::Array;

#[derive(serde::Deserialize, Clone)]
pub(crate) struct Config {
    // Remote URL
    pub(crate) remote: String,

    // Amount of backups to keep
    pub(crate) amount_of_backups_to_keep: u16,

    // SFTP Settings
    // SFTP public key path
    pub(crate) sftp_pubkey_path: Option<String>,
    pub(crate) sftp_privkey_path: Option<String>,
    pub(crate) sftp_privkey_password: Option<String>,
    // SFTP password
    pub(crate) sftp_password: Option<String>,

    // Path to folders to back up
    pub(crate) folders_to_backup: Array,
}

pub(crate) fn load_config(config_path: &str) -> Config {
    let config_path_final: &str = if config_path.starts_with("~") {
        let home_directory_raw = dirs::home_dir().expect("Could not retrieve home directory!");
        let home_dir = home_directory_raw
            .to_str()
            .expect("Could not convert home directory path object to str!");
        &config_path.replace("~", home_dir)
    } else {
        config_path
    };
    if fs::exists(config_path_final).is_err() {
        panic!("Could not find config path!");
    }
    let config_content = fs::read_to_string(config_path_final).expect("Could not read config file!");
    let mut config = toml::from_str(&config_content).expect("Could not parse config file!");
    config = check_config(config);
    config
}

fn check_config(config: Config) -> Config {
    for folder in config.clone().folders_to_backup {
        if folder.as_str().is_none() {
            panic!("Could not parse folder to backup: {:?}", folder);
        }
    }
    config
}
