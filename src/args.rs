use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(flatten)]
    pub(crate) verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,

    #[arg(short = 'c', long = "config")]
    pub(crate) config_file_path: Option<PathBuf>,
}
