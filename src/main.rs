mod archive;
mod upload;

mod args;
mod config;
mod intro;

use archive::tar::create_tarball;
use clap::Parser;
use config::{Config, load_config};
use env_logger::WriteStyle;
use intro::write_welcome_message;
use std::env;
use std::path::{Path, PathBuf};
use upload::upload_handler::{get_upload_mode, upload_file};

use crate::args::Args;
use crate::upload::upload_handler::UploadMode;

fn setup_logger(args: &Args) {
    const LOGGING_ENV: &str = "YRBA_LOG";

    let mut builder = env_logger::Builder::new();
    builder.filter_level(log::LevelFilter::Info);

    if env::var_os("NO_COLOR").is_some() {
        builder.write_style(WriteStyle::Never);
    } else {
        builder.write_style(WriteStyle::Auto);
    }

    if args.verbose.is_present() {
        builder.filter_level(args.verbose.log_level_filter());
    } else {
        builder.parse_env(LOGGING_ENV);
    }

    builder.init();
}

#[tokio::main]
async fn main() {
    // parse application args
    let args = Args::parse();
    setup_logger(&args);

    write_welcome_message();

    // load config file
    let config: Config = match load_config(args.config_file_path.as_ref()) {
        Ok(config) => config,
        Err(err) => {
            log::error!("Could not load config file!");
            log::debug!("Error: {err:?}");
            std::process::exit(1);
        }
    };

    let folders_to_backup: Vec<toml::Value> = config.folders_to_backup.clone();
    let upload_mode: UploadMode = get_upload_mode(&config.remote.clone()).unwrap_or_else(|err| {
        log::error!("Error determining upload mode!");
        log::debug!("Error: {err:?}");
        std::process::exit(2)
    });

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
                    log::info!("Created backup archive {}", temp_archive_path.display());
                    temp_archive_path
                }
                Err(err) => {
                    log::error!("Could not create archive {}", folder.display());
                    log::debug!("Error: {err:?}");
                    continue;
                }
            };

        // Uploading
        if let Err(err) = Box::pin(upload_file(&temp_archive_path, &upload_mode, &config)).await {
            log::error!("Upload failed: {err}");
            log::debug!("Error: {err:?}");
        }

        // Delete temporary archive
        if let Err(err) = std::fs::remove_file(&temp_archive_path) {
            log::error!(
                "Could not delete temporary archive! Please manually remove: {}",
                temp_archive_path.display()
            );
            log::debug!("Error: {err:?}");
        }
    }
}
