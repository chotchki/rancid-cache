use stabby::result::Result;
use stabby::string::String;

#[stabby::stabby(checked)]
pub trait RclTrait {
    extern "C" fn start(&self) -> Result<String, String>;
}

pub type RclPlugin = stabby::dynptr!(stabby::boxed::Box<dyn RclTrait>);

#[cfg(test)]
mod tests {
    use super::*;
    use stabby::boxed::Box;
    use stabby::result::Result;
    use stabby::string::String;

    struct RclTest;
    impl RclTrait for RclTest {
        extern "C" fn start(&self) -> Result<String, String> {
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
