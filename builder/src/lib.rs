use anyhow::{Result, bail};
use std::fs::{self, File, create_dir};
use std::io::{self, Write};
use std::path::PathBuf;
use std::{path::Path, process::Command};
use tempfile::tempdir;

pub mod cli;

pub fn build_module(cli: &cli::Cli) -> Result<PathBuf> {
    //Validate plugin output directory
    let output_path = Path::new(&cli.rcl_dir);
    if !output_path.try_exists()? {
        bail!("The output path does not exist {}", output_path.display());
    }

    let path_to_interface = format!("{}/../rcl", env!("CARGO_MANIFEST_DIR"));
    println!("rcl path is {}", path_to_interface);

    //Create the plugin as a simple cargo build, copy in the interface too
    //TODO: Harden this more
    let temp_compile_dir = tempdir()?;
    println!("temp path is {:?}", temp_compile_dir);

    let mut temp_path_cargo = PathBuf::new();
    temp_path_cargo.push(&temp_compile_dir);
    temp_path_cargo.push("Cargo.toml");
    let mut temp_cargo = File::create(temp_path_cargo)?;
    writeln!(
        temp_cargo,
        r#"
        [package]
        name = "builder_module"
        version = "0.1.0"
        edition = "2024"

        [lib]
        crate-type = ["cdylib"]

        [dependencies]
        rcl = {{ path = "{}" }}
        stabby = "72.1.1"
        "#,
        path_to_interface
    )?;

    let mut temp_path_src = PathBuf::new();
    temp_path_src.push(&temp_compile_dir);
    temp_path_src.push("src");
    create_dir(&temp_path_src)?;

    let mut temp_path_lib = PathBuf::new();
    temp_path_lib.push(temp_path_src);
    temp_path_lib.push("lib.rs");

    let mut temp_lib = File::create(temp_path_lib)?;

    //TODO: Unify these definitions
    writeln!(
        temp_lib,
        r#"
        use rcl::{{Rcl, RclPlugin}};
        use stabby::{{boxed::Box, result::Result, string::String}};

        struct RclTest {{
            pub inner: String
        }}
        impl Rcl for RclTest {{
            extern "C" fn start(&self) -> Result<String, String> {{
                Result::Ok("Works 2".into())
            }}
        }}

        #[stabby::export(canaries)]
        pub extern "C" fn rcl_plugin_init() -> Result<RclPlugin, String> {{
            println!("Inside the compiled constructor");
            let inner = String::new();
            let rt = RclTest {{ inner }};
            println!("Obj constructed");
            let box_rt = Box::new(rt);
            println!("Box constructed");
            let into_box_rt = box_rt.into();
            println!("Box intoed");
            Result::Ok(into_box_rt)
        }}
    "#
    )?;
    temp_lib.flush()?;

    //Now compile the plugin, we're going to cheat by making cargo do our dirty work
    //TODO: There is zero caching going on here so its slooooow
    let output = Command::new("cargo")
        .args(["build"])
        .current_dir(&temp_compile_dir)
        .output()?;

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    //Now copy the output out
    let mut temp_path_dynlib = PathBuf::new();
    temp_path_dynlib.push(&temp_compile_dir);
    temp_path_dynlib.push("target");
    temp_path_dynlib.push("debug");
    temp_path_dynlib.push("libbuilder_module.dylib");

    let mut output_path_file = PathBuf::new();
    output_path_file.push(output_path);
    output_path_file.push(format!("{}.rcl", cli.name));
    fs::copy(temp_path_dynlib, &output_path_file)?;

    Ok(output_path_file)
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
            rcl_dir: dir.path().to_path_buf(),
        };

        let output = build_module(&cli)?;
        assert!(output.exists());

        let mut output_path = cli.rcl_dir.clone();
        output_path.push("dummy.rcl");

        assert!(output_path.exists());

        Ok(())
    }
}
