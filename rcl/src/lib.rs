//! Rancid Cache Module Interface
//!
//! This is the crate defining how the coordinator should load and call modules built by rancid cache
#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// RCL Plugin Interface
///
/// This trait is used to allow a coordinator to know how to load and control configured modules. It is not intended to be directly used.
pub trait RclTrait {
    /// Starts up the module for processing, this signature will likely change a LOT
    fn start(&self) -> Result<String, String>;
}

/// This type is to allow for a construction function for the plugins, its TBD if the construction function needs to be mangled
pub type RclPlugin = Box<dyn RclTrait>;

#[cfg(test)]
mod tests {
    use super::*;

    struct RclTest;
    impl RclTrait for RclTest {
        fn start(&self) -> Result<String, String> {
            Result::Ok("Works".into())
        }
    }

    #[test]
    fn dummy_impl() {
        let test = RclTest;
        let output = test.start();

        assert_eq!(*"Works", *output.unwrap());
    }

    #[test]
    fn trait_test() {
        let obj = Box::new(RclTest);
        let dynobj = <RclPlugin>::from(obj);

        let output = dynobj.start();

        assert_eq!(*"Works", *output.unwrap());
    }
}
