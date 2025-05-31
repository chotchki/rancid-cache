use anyhow::Result;
use builder::build_module;
use builder::cli::Cli;
use clap::Parser;

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    build_module(&cli)?;

    Ok(())
}
