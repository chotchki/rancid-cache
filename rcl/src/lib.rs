pub trait RclTrait {
    fn start(&self) -> Result<String, String>;
}

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
