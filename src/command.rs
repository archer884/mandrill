use clap::App;
use error::*;
use std::env;
use std::result;
use std::str;

static MANDRILL_ENV_ARG_KEY: &str = "MANDRILL_API_KEY";

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

        if let Some(matches) = matches.subcommand_matches("inspect") {
            let key = matches.value_of("key")
                .map(|s| s.to_string())
                .or_else(|| get_key_from_env())
                .ok_or_else(Error::api_key)?;

            return Ok(Command {
                kind: CommandKind::Inspect,
                api_key: key,
                target: matches.value_of("target").unwrap().to_string(),
                vars: None,
            });
        }

        if let Some(matches) = matches.subcommand_matches("render") {
            let key = matches.value_of("key")
                .map(|s| s.to_string())
                .or_else(|| get_key_from_env())
                .ok_or_else(Error::api_key)?;

            let variables = matches.values_of("var")
                .map(|vars| vars.filter_map(|var| var.parse().ok()).collect());

            return Ok(Command {
                kind: CommandKind::Render,
                api_key: key,
                target: matches.value_of("target").unwrap().to_string(),
                vars: variables,
            })
        }

        if let Some(matches) = matches.subcommand_matches("fix") {
            let key = matches.value_of("key")
                .map(|s| s.to_string())
                .or_else(|| get_key_from_env())
                .ok_or_else(Error::api_key)?;

            return Ok(Command {
                kind: CommandKind::Fix,
                api_key: key,
                target: matches.value_of("target").unwrap().to_string(),
                vars: None,
            })
        }

        Err(Error::bad_command())
    }
}

fn get_key_from_env() -> Option<String> {
    env::vars()
        .filter(|&(ref k, _)| k == MANDRILL_ENV_ARG_KEY)
        .map(|(_, v)| v)
        .next()
}

fn build_app<'a, 'b>() -> App<'a, 'b> {
    clap_app!(mandrill =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "Inspect and correct mandrill templates")
        (@subcommand inspect =>
            (@arg target: +required + takes_value "The template to be inspected")
            (@arg key: +takes_value -k --key "The API key for your account")
        )
        (@subcommand fix =>
            (@arg target: +required +takes_value "The template to be fixed")
            (@arg key: +takes_value -k --key "The API key for your account")
        )
        (@subcommand render =>
            (@arg target: +required +takes_value "The template to be rendered")
            (@arg key: +takes_value -k --key "The API key for your account")
            (@arg var: +takes_value -r --var ... "A template replacement (<key>:<value>)")
        )
    )
}
