use std::borrow::Cow;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;
pub type Cause = Box<error::Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    cause: Option<Cause>,
    description: Cow<'static, str>,
}

#[derive(Debug)]
pub enum ErrorKind {
    ApiKey,
    BadCommand,
    Json(String),
    Other,
}

impl Error {
    pub fn api_key() -> Self {
        Error {
            kind: ErrorKind::ApiKey,
            cause: None,
            description: Cow::from("Missing API key"),
        }
    }

    pub fn bad_command() -> Self {
        Error {
            kind: ErrorKind::BadCommand,
            cause: None,
            description: Cow::from("No clue what you did, bro."),
        }
    }

    pub fn json(json: String) -> Self {
        let description = Cow::from(format!("Error parsing json:\n{}", json));
        Error {
            kind: ErrorKind::Json(json),
            cause: None,
            description,
        }
    }

    pub fn other<E: error::Error + 'static>(other: E) -> Self {
        Error {
            kind: ErrorKind::Other,
            cause: Some(Box::new(other)),
            description: Cow::from("An error occurred"),
        }
    }

    pub fn update(response: String) -> Self {
        Error {
            kind: ErrorKind::Other,
            cause: None,
            description: Cow::from(format!("An error occurred while updating:\n{}", response)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.cause {
            Some(ref e) => Some(e.as_ref()),
            None => None,
        }
    }
}
