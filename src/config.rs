use std::fs;

use toml::value::Array;

#[derive(serde::Deserialize)]
#[derive(Clone)]
pub(crate) struct Config {
    // Remote URL
    pub(crate) remote: String,

    // SFTP Settings
    // SFTP public key path
    pub(crate) sftp_pubkey_path: String,
    // SFTP password
    pub(crate) sftp_password: String,

    // Path to folders to back up
    pub(crate) folders_to_backup: Array,
}


pub(crate) fn load_config(config_path: &str) -> Config {
    if fs::exists(config_path).is_err() {
        panic!("Could not find config path!");
    }
    let config_content = fs::read_to_string(config_path).expect("Could not read config file!");
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

