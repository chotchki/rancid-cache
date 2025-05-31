use anyhow::{anyhow, bail};
use libloading::Library;
use rcl::RclDyn;
use rcl::{Rcl, RclPlugin};
use stabby::libloading::StabbyLibrary;
use std::{fs, path::Path};
pub mod cli;

pub fn load_and_run(cli: &cli::Cli) -> anyhow::Result<()> {
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
        unsafe {
            let lib = Library::new(path)?;

            let init_obj = lib
                .get_canaried::<extern "C" fn() -> stabby::result::Result<RclPlugin, stabby::string::String>>(b"rcl_plugin_init")
                .map_err(|e| anyhow!("Unable to get the constructor pointer: {}", e))?;
            println!("Got the constructor function");
            //let start_fn = lib
            //  .get_stabbied::<extern "C" fn(&RclPlugin) -> ()>(b"start")
            // .map_err(|e| anyhow!(e))?;

            let res_const = init_obj();

            if res_const.is_err() {
                bail!("Constructor failed! {}", res_const.err().unwrap());
            }

            let constructed = res_const.unwrap();
            println!("Constructed the trait object");

            let res_start = constructed.start();
            if res_start.is_err() {
                bail!("Unable to start! {}", res_start.err().unwrap());
            }

            println!("Started with value {}", res_start.unwrap());
        }
    }

    Ok(())
}
