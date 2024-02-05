use crate::args::Cli;
use crate::types::{Doctype, Program};
use reqwest::{get, Error};

static VERSION: &str = "v1";

// Check for the program in the database
// TODO implement haha
// TODO^2 add offline cache
pub async fn lookup(args: Cli) -> Result<Program, Error> {
    let name = &args.program.unwrap();
    let request_url = format!("https://glhf.shuttleapp.rs/api/{VERSION}/fetch/{name}");

    match get(request_url).await {
        Ok(response) => response.json().await,
        Err(e) => {
            println!("{}", e);
            Err(e)
        }
    }
}

// If there isn't an entry for the program, create it
pub async fn create(args: Cli) -> Result<Program, Error> {
    // TODO validation

    Ok(Program {
        program_name: args.program.unwrap(),
        url: None,
        doctype: Doctype::Man,
    })
}
