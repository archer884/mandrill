#![feature(specialization)]

#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;

extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod command;
mod error;
mod request;
mod response;
mod rest;

use command::*;
use error::Result;

fn main() {
    use std::error::Error;
    use std::process;

    let command = match Command::from_args() {
        Err(e) => {
            println!("{}", e.description());
            process::exit(1);
        }

        Ok(command) => command,
    };

    if let Err(e) = execute(command) {
        println!("{}", e.description());
        if let Some(cause) = e.cause() { println!("Cause: {}", cause.description()); }
        process::exit(2);
    }
}

fn execute(command: Command) -> Result<()> {
    match command.kind {
        CommandKind::Inspect => {
            let (code, _) = rest::inspect(&command)?;
            Ok({ println!("{}", code); })
        }
        
        CommandKind::Fix => rest::fix(&command),
        CommandKind::Render => rest::render(&command),
    }
}
