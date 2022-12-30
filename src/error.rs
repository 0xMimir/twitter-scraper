use chrono::ParseError;
use reqwest::StatusCode;
use serde::Deserialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    ReqwestError(String),
    SerdeError(String),
    ParseError(ParseError),
    UserSuspended,
    UserNotFound,
    Unauthorized,
    UnauthorizedToViewSpecificUser,
    RateLimitExceeded,
    UserUnavailable,

    #[non_exhaustive]
    UnknownError,
}

impl From<StatusCode> for Error {
    fn from(value: StatusCode) -> Self {
        match value.as_u16() {
            429 => Self::RateLimitExceeded,
            _ => Self::UnknownError,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        if let Some(status) = error.status() {
            let error_from_status = Self::from(status);
            if error_from_status != Self::UnknownError {
                return error_from_status;
            }
        }
        Self::ReqwestError(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeError(error.to_string())
    }
}

impl From<ParseError> for Error {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}

impl From<serde_url_params::Error> for Error {
    fn from(value: serde_url_params::Error) -> Self {
        Self::SerdeError(value.to_string())
    }
}

#[derive(Deserialize)]
pub struct ResponseError {
    pub errors: Vec<Message>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub code: i32,
}

impl From<ResponseError> for Error {
    fn from(value: ResponseError) -> Self {
        match value.errors.first() {
            Some(msg) => match msg.code {
                22 => Self::UnauthorizedToViewSpecificUser,
                37 => Self::Unauthorized,
                50 => Self::UserNotFound,
                63 => Self::UserSuspended,
                _ => Self::UnknownError,
            },
            None => Self::UnknownError,
        }
    }
}
