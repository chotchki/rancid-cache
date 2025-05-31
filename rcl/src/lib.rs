#[stabby::stabby(checked)]
pub trait Rcl {
    extern "C" fn start(&self);
}
