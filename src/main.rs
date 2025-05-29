use anyhow::Result;
use clap::Parser;
use cli::Cli;
use rcgen::{CertifiedKey, generate_simple_self_signed};
use std::{net::SocketAddr, path::Path};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod vcl;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    //The concept is on start up, take an argument for the listener port and compile / load a "VCL" backend.
    //Features don't matter here as much as exploring the runtime loading.
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=trace,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0], cli.port));
    tracing::debug!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    //Check for certificate directory
    let cert_path = Path::new(&cli.certificate_dir);
    if !cert_path.try_exists()? {
        tracing::error!(
            "Certificate directory: {} doesn't exist",
            cert_path.display()
        );
    }

    //Load / Generate Certificate
    let subject_alt_names = vec!["localhost".to_string()];
    let CertifiedKey { cert, key_pair } = generate_simple_self_signed(subject_alt_names).unwrap();

    //Show the certificate
    println!("{}", cert.pem());
    println!("{}", key_pair.serialize_pem());

    Ok(())
}
