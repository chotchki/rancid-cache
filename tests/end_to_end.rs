extern crate anyhow;
extern crate builder;
extern crate coordinator;
extern crate tempfile;

use anyhow::Result;
use builder::cli::Cli;
use coordinator::load_and_run;
use tempfile::tempdir;

#[test]
fn end_to_end() -> Result<()> {
    let rcl_dir = tempdir()?;

    let builder_config = builder::cli::Cli {
        name: "end_to_end".to_string(),
        rcl_dir: rcl_dir.path().to_path_buf(),
    };

    let output = builder::build_module(&builder_config)?;
    println!("The build completed.");

    let coordinator_config = coordinator::cli::Cli {
        rcl_dir: rcl_dir.path().to_path_buf(),
    };
    load_and_run(&coordinator_config)?;
    println!("The load and run completed.");

    Ok(())
}
