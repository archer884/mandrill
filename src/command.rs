use clap::App;
use error::*;
use std::env;
use std::result;
use std::str;

static MANDRILL_SECRET_PATH_KEY: &str = "MANDRILL_SECRET_PATH";

#[derive(Debug, Serialize)]
pub struct VariableReplacement {
    name: String,
    content: String,
}

impl str::FromStr for VariableReplacement {
    type Err = ();

    fn from_str(s: &str) -> result::Result<Self, ()> {
        let mut parts = s.split(':');
        Ok(VariableReplacement {
            name: parts.next().map(|s| s.to_string()).ok_or(())?,
            content: parts.next().map(|s| s.to_string()).ok_or(())?,
        })
    }
}

#[derive(Debug)]
pub struct Command {
    pub kind: CommandKind,
    pub api_key: String,
    pub target: String,
    pub vars: Option<Vec<VariableReplacement>>,
}

#[derive(Copy, Clone, Debug)]
pub enum CommandKind {
    Inspect,
    Fix,
    Render,
}

impl Command {
    pub fn from_args() -> Result<Self> {
        let matches = build_app().get_matches();
        let api_key = read_key()?;

        if let Some(matches) = matches.subcommand_matches("inspect") {
            return Ok(Command {
                kind: CommandKind::Inspect,
                api_key,
                target: matches.value_of("target").unwrap().to_string(),
                vars: None,
            });
        }

        if let Some(matches) = matches.subcommand_matches("render") {
            let variables = matches.values_of("var")
                .map(|vars| vars.filter_map(|var| var.parse().ok()).collect());

            return Ok(Command {
                kind: CommandKind::Render,
                api_key,
                target: matches.value_of("target").unwrap().to_string(),
                vars: variables,
            })
        }

        if let Some(matches) = matches.subcommand_matches("fix") {
            return Ok(Command {
                kind: CommandKind::Fix,
                api_key,
                target: matches.value_of("target").unwrap().to_string(),
                vars: None,
            })
        }

        Err(Error::bad_command())
    }
}

fn build_app<'a, 'b>() -> App<'a, 'b> {
    clap_app!(mandrill =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Inspect and correct mandrill templates")
        (@subcommand inspect =>
            (@arg target: +required + takes_value "The template to be inspected")
        )
        (@subcommand fix =>
            (@arg target: +required +takes_value "The template to be fixed")
        )
        (@subcommand render =>
            (@arg target: +required +takes_value "The template to be rendered")
            (@arg var: +takes_value -r --var ... "A template replacement (<key>:<value>)")
        )
    )
}

fn read_secret_path() -> Option<String> {
    env::vars().filter(|arg| arg.0 == MANDRILL_SECRET_PATH_KEY).map(|arg| arg.1).next()
}

fn read_key() -> Result<String> {
    use std::fs::File;
    use std::io::{Read, BufReader};

    let path = read_secret_path().ok_or_else(Error::api_key)?;
    match File::open(&path).map(BufReader::new) {
        Err(_) => {
            eprintln!("It is unwise to load keys directly; try `MANDRILL_SECRET_PATH=<(cat <key file path>) mandrill <args>` instead");
            Ok(path.to_string())
        }

        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf).ok();

            let len = buf.rfind(|c| !char::is_whitespace(c)).map(|len| len + 1).unwrap_or(0);
            buf.truncate(len);

            Ok(buf)
        }
    }
}
