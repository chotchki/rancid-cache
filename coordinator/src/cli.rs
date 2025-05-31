use std::path::PathBuf;

use clap::{Parser, command};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub rcl_dir: PathBuf,
}
