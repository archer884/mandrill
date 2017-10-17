use command::Command;
use error::*;
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
        .send()
        .map_err(|e| Error::other(e))?;

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
        .send()
        .map_err(|e| Error::other(e))?;

    let response = read_response_to_string(response);
    let parsed_response: RenderResponse = json::from_str(&response)
        .map_err(|_| Error::json(response))?;

    Ok({ println!("{}", parsed_response.html); })
}

pub fn fix(command: &Command) -> Result<()> {
    use regex::Regex;
    use reqwest::StatusCode;

    let (code, text) = inspect(&command)?;
    let pattern = Regex::new(r#"\*\|MC_[A-Z_]+\|\*"#).unwrap();

    if pattern.is_match(&code) {
        let code = pattern.replace_all(&code, "");
        let text = text.as_ref().map(|text| pattern.replace_all(&text, ""));
        let request = UpdateRequest::new(&command, &code, text.as_ref().map(|s| s.as_ref()));

        let response = Client::new()
            .post(MANDRILL_UPDATE_URI)
            .header(ContentType::json())
            .body(json::to_string(&request).unwrap())
            .send()
            .map_err(|e| Error::other(e))?;

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
