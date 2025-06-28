use anyhow::{anyhow, bail};
use libloading::Library;
use plugin_interface::Plugin;
use std::fs;
pub mod cli;

pub fn load_and_run(cli: &cli::Cli) -> anyhow::Result<()> {
    println!("Inside load and run");
    let mut paths = vec![];
    for entry in fs::read_dir(&cli.rcl_dir)? {
        let dir = entry?;
        if dir.file_name().to_string_lossy().ends_with(".rcl") {
            paths.push(dir.path());
        }
    }

    //For each entry we're going to execute it... no hardening here, just make it go boom!
    for path in paths {
        //TODO need to document the safety concerns and how we can handle/guard it
        println!("Trying to use {path:?}",);
        unsafe {
            let lib = Library::new(path)?;

            let init_obj: libloading::Symbol<unsafe fn() -> Result<Plugin, String>> = lib
                .get(b"plugin_init")
                .map_err(|e| anyhow!("Unable to get the constructor pointer: {}", e))?;
            println!("Got the constructor function");

            let res_const = init_obj();

            if res_const.is_err() {
                bail!("Constructor failed! {}", res_const.err().unwrap());
            }

            let constructed = res_const.unwrap();
            println!("Constructed the trait object");

            constructed.start();
            println!("Started library");
        }
    }

    Ok(())
}
