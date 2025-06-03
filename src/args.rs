use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity<clap_verbosity_flag::InfoLevel>,

    #[args(short = 'c', long, default_value = "~/.config/yrba/config.toml")]
    pub(crate) config_file_path: String,
}

pub (crate) fn setup_logging() -> Args {
    let args: Args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    args
}

