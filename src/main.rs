mod upload;
mod archive;

mod config;
mod args;

use archive::tar::create_tarball;
use config::{load_config, Config};


use crate::args::{Args, setup_logging};

fn main() {
    println!("Hello, world!");
    // parse application args
    let args: Args = setup_logging();

    // load config file
    let config: Config = load_config(&args.config_file_path);

    let folders_to_backup: Vec<toml::Value> = config.folders_to_backup;

    for folder_raw in folders_to_backup {
        println!("{:?}", folder_raw);
        let folder = std::path::Path::new(folder_raw
            .as_str()
            .expect("`folders_to_backup` is checked during loading of config file"));
        match create_tarball(Box::new(folder)) {
            Ok(temp_archive_path) => log::info!("Created archive {:?}", temp_archive_path),
            Err(err) => log::error!("Could not created archive {:?}\nError: {:?}", folder, err)
        }
    }
}
