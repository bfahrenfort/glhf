use clap::ValueEnum;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
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
