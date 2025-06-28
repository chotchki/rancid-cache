//! Rancid Cache Plugin Interface
//!
//! This is the crate defining how the coordinator should load and call modules built by rancid cache
#![forbid(unsafe_code)]
use async_trait::async_trait;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncWrite};

pub trait AsyncStreamInterface: AsyncRead + AsyncWrite + Send {}
impl<T> AsyncStreamInterface for T where T: AsyncRead + AsyncWrite + Send {}

/// Plugin Interface
///
/// This trait is used to allow a coordinator to know how to load and control configured modules. It is not intended to be directly used outside this codebase.
#[async_trait]
pub trait PluginInterface {
    fn start(&self);

    async fn handle_connection(
        &self,
        stream: Box<dyn AsyncStreamInterface>,
    ) -> Result<(), PluginError>;
}

/// Errors that should be communicated back to the coordinator
#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Handling Error {0}")]
    Handling(String),
}

/// This type is to allow for a construction function for the plugins, its TBD if the construction function needs to be mangled
pub type Plugin = Box<dyn PluginInterface>;

#[cfg(test)]
mod tests {
    use tokio_test::io::Builder;

    use super::*;

    struct PluginTest;
    #[async_trait]
    impl PluginInterface for PluginTest {
        fn start(&self) {}
        async fn handle_connection(
            &self,
            _stream: Box<dyn AsyncStreamInterface>,
        ) -> Result<(), PluginError> {
            println!("Connection handled");
            Ok(())
        }
    }

    #[tokio::test]
    async fn dummy_impl() -> Result<(), anyhow::Error> {
        let test = PluginTest;
        test.start();

        let mut builder = Builder::new();
        let mock = Box::new(builder.build());

        test.handle_connection(mock).await?;
        Ok(())
    }

    #[tokio::test]
    async fn trait_test() -> Result<(), anyhow::Error> {
        let obj = Box::new(PluginTest);
        let dynobj = <Plugin>::from(obj);

        dynobj.start();

        let mut builder = Builder::new();
        let mock = Box::new(builder.build());

        dynobj.handle_connection(mock).await?;
        Ok(())
    }
}
