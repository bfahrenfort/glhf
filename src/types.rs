use clap::ValueEnum;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Deserialize)]
pub enum Doctype {
    Book,
    Github,
    Man,
    Unspecified,
}
impl fmt::Display for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug)]
pub struct Program {
    pub program_name: String,
    pub doctype: Doctype,
    pub url: Option<String>,
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.program_name,
            self.doctype,
            self.url.clone().unwrap()
        )
    }
}
