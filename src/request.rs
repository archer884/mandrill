use command::*;

static MERGE_LANGUAGE: &str = "handlebars";
static TEMPLATE_CONTENT: &[(String, String)] = &[];

#[derive(Serialize)]
pub struct InfoRequest<'a> {
    key: &'a str,
    name: &'a str,
}

impl<'a> InfoRequest<'a> {
    pub fn from_command(command: &'a Command) -> Self {
        Self {
            key: &command.api_key,
            name: &command.target,
        }
    }
}

#[derive(Serialize)]
pub struct RenderRequest<'a> {
    pub key: &'a str,
    pub template_name: &'a str,
    pub merge_language: &'static str,
    pub merge_vars: Option<&'a [VariableReplacement]>,
    pub template_content: &'static [(String, String)],
}

impl<'a> RenderRequest<'a> {
    pub fn from_command(command: &'a Command) -> Self {
        Self {
            key: &command.api_key,
            template_name: &command.target,
            merge_language: MERGE_LANGUAGE,
            merge_vars: command.vars.as_ref().map(|vars| vars.as_ref()),
            template_content: TEMPLATE_CONTENT,
        }
    }
}

#[derive(Serialize)]
pub struct UpdateRequest<'a> {
   pub key: &'a str,
   pub name: &'a str,
   pub code: &'a str,
   pub text: Option<&'a str>,
   pub publish: bool,
}

impl<'a> UpdateRequest<'a> {
    pub fn new(command: &'a Command, code: &'a str, text: Option<&'a str>) -> Self {
        Self { 
            key: &command.api_key,
            name: &command.target,
            code,
            text,
            publish: true
        }
    }
}
