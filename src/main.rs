mod archive;
mod upload;

mod args;
mod config;
mod intro;

use archive::tar::create_tarball;
use config::{Config, load_config};
use intro::write_welcome_message;
use std::path::{Path, PathBuf};
use upload::upload_handler::{get_upload_mode, upload_file};

use crate::args::{Args, setup_logging};
use crate::upload::upload_handler::UploadMode;

fn main() {
    // parse application args
    let args: Args = setup_logging();

    write_welcome_message();

    // load config file
    let config: Config = load_config(args.config_file_path.as_ref());

    let folders_to_backup: Vec<toml::Value> = config.folders_to_backup.clone();
    let upload_mode: UploadMode = get_upload_mode(&config.remote.clone());

    for folder_raw in folders_to_backup {
        log::info!("Backup started for: {folder_raw}");
        // Archiving
        log::debug!("Archiving...");
        let folder: &Path = Path::new(
            folder_raw
                .as_str()
                .expect("`folders_to_backup` is checked during loading of config file"),
        );
        let temp_archive_path: PathBuf =
            match create_tarball(folder, config.clone().temporary_folder) {
                Ok(temp_archive_path) => {
                    log::info!("Created archive {}", temp_archive_path.display());
                    temp_archive_path
                }
                Err(err) => {
                    log::error!(
                        "Could not create archive {}\nError: {err:?}",
                        folder.display()
                    );
                    continue;
                }
            };

        // Uploading
        upload_file(&temp_archive_path, &upload_mode, &config);

        // Delete temporary archive
        if std::fs::remove_file(&temp_archive_path).is_err() {
            log::error!(
                "Could not delete temporary archive! Please manually remove: {:?}",
                temp_archive_path.display()
            );
        }
    }
}
