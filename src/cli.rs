use clap::{Parser, command};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub certificate_dir: String,

    pub vcl_dir: String,

    #[arg(default_value_t = 3000)]
    pub port: u16,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
