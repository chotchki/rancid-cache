extern crate builder;
extern crate coordinator;
extern crate tempfile;
extern crate tokio;

use anyhow::Result;
use tempfile::tempdir;

#[tokio::test]
async fn basic_backend() -> Result<()> {
    //Will compile and load a single RCL backend and we should be able to hit google with it

    let basic_backend = "backend default {
        .host = \"google.com:443\";
    }";

    let rcl_dir = tempdir()?;

    let builder_config = builder::cli::Cli {
        name: "basic_backend".to_string(), //Name doesn't really make any sense here anymore
        rcl_dir: rcl_dir.path().to_path_buf(),
    };

    let module_builder = builder::new(builder_config)?;
    module_builder.create_module("basic_backend", basic_backend)?; //Should this return a module? Not sure it makes sense at all

    let coordinator_config = coordinator::cli::Cli {
        rcl_dir: rcl_dir.path().to_path_buf(),
    };
    let coordinator = coordinator::load(coordinator_config).await?;

    let coordinator_handle = tokio::spawn(async move {
        //Need to figure out how to find the port out, maybe come back in with the config again?
        coordinator.start().await?;
    });

    todo!("Make a request to the coordinator and ensure something google like comes out");

    Ok(())
}
