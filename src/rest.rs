use command::Command;
use error::*;
use regex::Regex;
use request::*;
use reqwest::Client;
use reqwest::header::ContentType;
use response::*;
use serde_json as json;
use std::io::Read;

static MANDRILL_INFO_URI: &str = "https://mandrillapp.com/api/1.0/templates/info.json";
static MANDRILL_RENDER_URI: &str = "https://mandrillapp.com/api/1.0/templates/render.json";
static MANDRILL_UPDATE_URI: &str = "https://mandrillapp.com/api/1.0/templates/update.json";

pub fn inspect(command: &Command) -> Result<(String, Option<String>)> {
    let response = Client::new()
        .post(MANDRILL_INFO_URI)
        .header(ContentType::json())
        .body(json::to_string(&InfoRequest::from_command(command)).unwrap())
        .send()?;

    let response = read_response_to_string(response);
    let parsed_response: InfoResponse = json::from_str(&response)
        .map_err(|_| Error::json(response))?;

    Ok((parsed_response.code, parsed_response.text))
}

pub fn render(command: &Command) -> Result<()> {
    let response = Client::new()
        .post(MANDRILL_RENDER_URI)
        .header(ContentType::json())
        .body(json::to_string(&RenderRequest::from_command(command)).unwrap())
        .send()?;

    let response = read_response_to_string(response);
    let parsed_response: RenderResponse = json::from_str(&response)
        .map_err(|_| Error::json(response))?;

    Ok({ println!("{}", parsed_response.html) })
}

pub fn fix(command: &Command) -> Result<()> {
    use reqwest::StatusCode;

    let (code, text) = inspect(command)?;
    let template_pattern = template_pattern();
    let link_pattern = link_pattern();

    if template_pattern.is_match(&code) || link_pattern.is_match(&code) {
        let code = template_pattern.replace_all(&code, "");
        let code = link_pattern.replace_all(&code, "{{link}}");
        
        let text = text.as_ref().map(|text| template_pattern.replace_all(text, ""));
        let text = text.as_ref().map(|text| link_pattern.replace_all(text, "{{link}}"));

        let request = UpdateRequest::new(command, &code, text.as_ref().map(|s| s.as_ref()));

        let response = Client::new()
            .post(MANDRILL_UPDATE_URI)
            .header(ContentType::json())
            .body(json::to_string(&request).unwrap())
            .send()?;

        if response.status() == StatusCode::Ok {
            println!("updated {}", command.target);
        } else {
            return Err(Error::update(read_response_to_string(response)));
        }
    }
    
    Ok(())
}

fn read_response_to_string<R: Read>(mut response: R) -> String {
    let mut buf = String::new();
    response.read_to_string(&mut buf).ok();
    buf
}

fn template_pattern() -> Regex {
    Regex::new(r#"\*\|MC_[A-Z_]+\|\*"#).unwrap()
}

fn link_pattern() -> Regex {
    Regex::new(r#"http://\{\{link\}\}"#).unwrap()
}

#[cfg(test)]
mod regex_pattern_tests {    
    #[test]
    fn link_pattern_is_valid() {
        super::link_pattern();
    }

    #[test]
    fn template_pattern_is_valid() {
        super::template_pattern();
    }
}
