mod upload;
mod archive;

mod config;
mod args;

use crate::args::{Args, setup_logging};

fn main() {
    println!("Hello, world!");
    let args: Args = setup_logging();
}
