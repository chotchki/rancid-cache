//! Rancid Cache Module Interface
//!
//! This is the crate defining how the coordinator should load and call modules built by rancid cache
#![forbid(unsafe_code)]
use async_trait::async_trait;
use thiserror::Error;
use tokio::io::{AsyncRead, AsyncWrite};

pub trait AsyncStreamInterface: AsyncRead + AsyncWrite + Send {}
impl<T> AsyncStreamInterface for T where T: AsyncRead + AsyncWrite + Send {}

/// RCL Plugin Interface
///
/// This trait is used to allow a coordinator to know how to load and control configured modules. It is not intended to be directly used outside this codebase.
#[async_trait]
pub trait RclTrait {
    fn start(&self);

    async fn handle_connection(
        &self,
        stream: Box<dyn AsyncStreamInterface>,
    ) -> Result<(), RclPluginError>;
}

/// Errors that should be communicated back to the coordinator
#[derive(Error, Debug)]
pub enum RclPluginError {
    #[error("Handling Error {0}")]
    Handling(String),
}

/// This type is to allow for a construction function for the plugins, its TBD if the construction function needs to be mangled
pub type RclPlugin = Box<dyn RclTrait>;

#[cfg(test)]
mod tests {
    use tokio_test::io::Builder;

    use super::*;

    struct RclTest;
    #[async_trait]
    impl RclTrait for RclTest {
        fn start(&self) {}
        async fn handle_connection(
            &self,
            _stream: Box<dyn AsyncStreamInterface>,
        ) -> Result<(), RclPluginError> {
            println!("Connection handled");
            Ok(())
        }
    }

    #[tokio::test]
    async fn dummy_impl() -> Result<(), anyhow::Error> {
        let test = RclTest;
        test.start();

        let mut builder = Builder::new();
        let mock = Box::new(builder.build());

        test.handle_connection(mock).await?;
        Ok(())
    }

    #[tokio::test]
    async fn trait_test() -> Result<(), anyhow::Error> {
        let obj = Box::new(RclTest);
        let dynobj = <RclPlugin>::from(obj);

        dynobj.start();

        let mut builder = Builder::new();
        let mock = Box::new(builder.build());

        dynobj.handle_connection(mock).await?;
        Ok(())
    }
}
