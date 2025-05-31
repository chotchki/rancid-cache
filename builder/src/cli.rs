use clap::{Parser, command};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub name: String,
    pub rcl_dir: String,
}
