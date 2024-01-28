use clap::{Args, Parser, Subcommand};

use crate::types::Doctype;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// The name of the program for use with glhf
    pub program: String,

    /// Documentation type
    pub doctype: Option<Doctype>,

    /// Whether to contribute this program to glhf's database if it doesn't exist
    #[arg(short, long, default_value_t = false)]
    pub create: bool,

    /// [Optional] The URL of the Github or Book documentation
    pub url: Option<String>,

    /// [Alternate] command-style syntax:
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Create(CreateArgs),
}

#[derive(Args)]
pub struct CreateArgs {
    /// Name of the program to create an entry for
    pub program: String,

    /// Type of documentation attached
    pub doctype: Option<Doctype>,

    /// URL where that documentation lies
    pub url: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
