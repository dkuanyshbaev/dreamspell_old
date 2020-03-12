use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::Responder;
use std::convert::From;
use std::{error, fmt};

#[derive(Debug)]
pub enum DreamError {
    NotFound,
    InternalServerError,
    BadRequest,
    Unauthorized,
}

impl fmt::Display for DreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DreamError::NotFound => write!(f, "NotFound"),
            DreamError::InternalServerError => write!(f, "InternalServerError"),
            DreamError::BadRequest => write!(f, "BadRequest"),
            DreamError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl error::Error for DreamError {
    fn description(&self) -> &str {
        match *self {
            DreamError::NotFound => "Record not found",
            DreamError::InternalServerError => "Internal server error",
            DreamError::BadRequest => "Bad Request",
            DreamError::Unauthorized => "Unauthorized",
        }
    }
}

impl From<DieselError> for DreamError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::NotFound => DreamError::NotFound,
            _ => DreamError::InternalServerError,
        }
    }
}

impl<'r> Responder<'r> for DreamError {
    fn respond_to(self, _: &Request) -> rocket::response::Result<'r> {
        match self {
            DreamError::NotFound => Err(Status::NotFound),
            DreamError::BadRequest => Err(Status::BadRequest),
            DreamError::Unauthorized => Err(Status::Unauthorized),
            _ => Err(Status::InternalServerError),
        }
    }
}
