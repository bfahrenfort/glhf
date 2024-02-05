mod args;
mod invoke;
mod types;

use std::io::{self, Write};
use std::process::{Command, Output};

use args::{Cli, Commands};
use invoke::{create, lookup};
use reqwest::StatusCode;
use types::{Doctype, Program};

#[tokio::main]
async fn main() {
    let args = args::parse();

    println!(
        "{} {} {}",
        args.program.clone().unwrap_or("none".to_string()).as_str(),
        args.doctype.unwrap_or(Doctype::Unspecified),
        args.create
    );

    // TODO input validation

    let name = args.program.clone().unwrap(); // Need an unmoved string for lower down
    let fetched = match args.command {
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

    match fetched {
        Ok(response) => {
            // Open docs
            //
            // Only one Program
            // For now, response: Program. TODO Vec<Program> in front and backend
            println!(
                "Found program {} with doctype {}",
                response.program_name.clone(),
                response.doctype
            );
            let process = open_docs(response, true);

            // Multiple Programs (correspondingly multiple doctypes)
            // TODO
        }
        Err(e) => {
            println!("{}", e);

            // If it wasn't in the db
            if e.status() == Some(StatusCode::NOT_FOUND) {
                println!("glhf: {} not found.", name);
            }
        }
    }
}

fn open_docs(program: Program, silent: bool) -> Result<(), std::io::Error> {
    if program.doctype == Doctype::Man {
        if !silent {
            println!("man {}", program.program_name.clone());
        }
        Command::new("man")
            .arg(program.program_name)
            .spawn()
            .expect("glhf: could not open manpage");
        Ok(())
    } else {
        // It's a link-style docs
        if !silent {
            println!("xdg-open {}", program.url.clone().unwrap());
        }
        let process = Command::new("xdg-open")
            .arg(program.url.unwrap())
            .output()
            .expect("glhf: could not open browser. is `xdg-open` installed?");
        io::stdout().write_all(&process.stdout).unwrap();
        io::stderr().write_all(&process.stdout).unwrap();
        Ok(())
    }
}
