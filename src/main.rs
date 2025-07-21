mod archive;
mod upload;

mod args;
mod config;
mod intro;

use archive::tar::create_tarball;
use config::{Config, load_config};
use intro::write_welcome_message;
use upload::upload_handler::{get_upload_mode, upload_file};

use crate::args::{Args, setup_logging};

fn main() {
    // parse application args
    let args: Args = setup_logging();

    write_welcome_message();

    // load config file
    let config: Config = load_config(&args.config_file_path);

    let folders_to_backup: Vec<toml::Value> = config.folders_to_backup.clone();
    let upload_mode = get_upload_mode(config.remote.clone());

    for folder_raw in folders_to_backup {
        // Archiving
        log::info!("Archiving: {}", folder_raw);
        let folder = std::path::Path::new(
            folder_raw
                .as_str()
                .expect("`folders_to_backup` is checked during loading of config file"),
        );
        let temp_archive_path = match create_tarball(folder, config.clone().temporary_folder) {
            Ok(temp_archive_path) => {
                log::info!("Created archive {:?}", temp_archive_path);
                temp_archive_path
            }
            Err(err) => {
                log::error!("Could not create archive {:?}\nError: {:?}", folder, err);
                continue;
            }
        };

        // Uploading
        upload_file(temp_archive_path, upload_mode.clone(), config.clone());
    }
}
