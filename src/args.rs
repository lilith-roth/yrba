use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,

    #[arg(short = 'c', long = "config")]
    pub(crate) config_file_path: Option<PathBuf>,
}

pub(crate) fn setup_logging() -> Args {
    let args: Args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    args
}
