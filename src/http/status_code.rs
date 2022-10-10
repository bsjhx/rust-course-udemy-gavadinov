use std::fmt::Display;

#[derive(Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            StatusCode::Ok => "Ok",
            StatusCode::BadRequest => "Bad request",
            StatusCode::NotFound => "Not found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}