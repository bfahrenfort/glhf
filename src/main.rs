mod args;
mod invoke;
mod types;

use args::{Cli, Commands};
use invoke::{create, lookup};
use tokio::main;
use types::Doctype;

#[tokio::main]
async fn main() {
    let args = args::parse();

    println!(
        "{} {} {}",
        args.program.clone().unwrap_or("none".to_string()).as_str(),
        args.doctype.unwrap_or(Doctype::Unspecified),
        args.create
    );

    // if let Ok(program) = match args.command {
    let program = match args.command {
        None => lookup(args).await,
        Some(Commands::Create(c)) => {
            let args = Cli {
                program: Some(c.program),
                doctype: c.doctype,
                create: true,
                url: c.url,
                command: None,
            };
            create(args).await
        }
    };

    match program {
        Ok(program) => {
            // open docs
            println!("result: {}", program);
            ()
        }
        Err(program) => {
            // Ask to create
            println!("{}", program);
            ()
        }
    }
}
