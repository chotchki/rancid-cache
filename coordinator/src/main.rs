use anyhow::Result;
use clap::Parser;
use coordinator::cli::Cli;

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    //Now we're going to load each module and execute it

    Ok(())
}
