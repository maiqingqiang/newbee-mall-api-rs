use std::fmt::Debug;
use std::num::ParseIntError;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use log::error;

use crate::bootstrap::response::Response;

#[derive(Debug, Display, Error)]
#[display(fmt = "{}", message)]
pub struct ApplicationError {
    pub(crate) status: StatusCode,
    pub(crate) message: String,
}

impl ApplicationError {
    pub fn error(message: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message,
        }
    }
}

impl ResponseError for ApplicationError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        error!(
            "response error status:{} message:{}",
            self.status_code().as_u16(),
            self
        );
        Response::new(self.status_code(), self.to_string().as_str(), ())
    }
}

impl From<diesel::result::Error> for ApplicationError {
    fn from(error: diesel::result::Error) -> Self {
        Self::error(error.to_string())
    }
}

impl From<r2d2::Error> for ApplicationError {
    fn from(error: r2d2::Error) -> Self {
        Self::error(error.to_string())
    }
}

impl From<String> for ApplicationError {
    fn from(error: String) -> Self {
        Self::error(error)
    }
}

impl From<&str> for ApplicationError {
    fn from(error: &str) -> Self {
        Self::error(error.to_string())
    }
}

impl From<actix_web::Error> for ApplicationError {
    fn from(error: actix_web::Error) -> Self {
        Self {
            status: error.as_response_error().status_code(),
            message: error.to_string(),
        }
    }
}

impl From<ParseIntError> for ApplicationError {
    fn from(error: ParseIntError) -> Self {
        Self::error(error.to_string())
    }
}
