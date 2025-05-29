pub mod generator;
pub mod parser;

#[derive(Debug, PartialEq)]
pub struct Backend {
    pub name: String,
    pub host: String,
}
