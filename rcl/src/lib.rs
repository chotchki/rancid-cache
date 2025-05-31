#[stabby::stabby(checked)]
pub trait Rcl {
    extern "C" fn start(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RclTest;
    impl Rcl for RclTest {
        extern "C" fn start(&self) {
            println!("Works");
        }
    }

    #[test]
    fn dummy_impl() {
        let test = RclTest;
        test.start();
    }
}
