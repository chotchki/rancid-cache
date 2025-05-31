use anyhow::{Result, bail};
use std::io::{self, Write};
use std::{path::Path, process::Command};
use tempfile::NamedTempFile;

pub mod cli;

pub fn build_module(cli: &cli::Cli) -> Result<()> {
    //Validate plugin output directory
    let output_path = Path::new(&cli.rcl_dir);
    if !output_path.try_exists()? {
        bail!("The output path does not exist {}", output_path.display());
    }

    //Our base approach is to write a super simple rust program out to a temp directory that conforms to the rcl interface
    //TODO: Clean up this approach, the tempfile isn't secure
    let mut plugin_src = NamedTempFile::new()?;
    writeln!(
        plugin_src,
        r#"
        pub fn main() {{
            println!("Hello World");
    }}
    "#
    )?;
    plugin_src.flush()?;

    //Now compile the plugin
    //TODO: Remove the unwrap
    let output = Command::new("rustc")
        .args([
            plugin_src.path().as_os_str().to_str().unwrap(),
            "--crate-name",
            "builder",
            "-o",
            format!("{}/{}.rcl", cli.rcl_dir, cli.name).as_str(),
        ])
        .output()?;

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use crate::cli::Cli;

    use super::*;

    #[test]
    fn simple_module() -> Result<()> {
        let dir = tempdir()?;

        let cli = Cli {
            name: "dummy".to_string(),
            rcl_dir: dir.path().to_string_lossy().to_string(),
        };

        build_module(&cli)?;

        let output_file = format!("{}/dummy.rcl", cli.rcl_dir);
        assert!(Path::new(&output_file).exists());

        Ok(())
    }
}
