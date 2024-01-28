mod args;
mod invoke;
mod types;

use args::{Cli, Commands};
use invoke::lookup;
use types::Doctype;

fn main() {
    let args = args::parse();

    println!(
        "{} {} {}",
        args.program.as_str(),
        args.doctype.unwrap_or(Doctype::Unspecified),
        args.create
    );

    match args.command {
        None => lookup(args),
        Some(Commands::Create(c)) => {
            let args = Cli {
                program: c.program,
                doctype: c.doctype,
                create: true,
                url: c.url,
                command: None,
            };
            lookup(args);
        }
    };
}
