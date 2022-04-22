use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_new::new;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The username should be smaller than 30 character")]
    /// The username should be smaller than 30 character
    LongUsername,
    #[error("There is no endpoint in this path")]
    /// There is no endpoint in this path
    NotFound,
    #[error("{0}")]
    /// Bad request ( error message will be in the field )
    BadRequest(String),
}

#[derive(Serialize, Deserialize, new)]
pub struct ErrorResponse {
    #[new(value = "false")]
    /// Status of response ( false with errors )
    pub status: bool,
    /// Response code
    pub code: u16,
    /// Error name
    pub error: String,
    /// Error message
    pub message: String,
}

impl AppError {
    /// Get error name
    pub fn name(&self) -> String {
        match self {
            Self::LongUsername => "LongUsername".to_owned(),
            Self::NotFound => "NotFound".to_owned(),
            Self::BadRequest(_) => "BadRequest".to_owned(),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::LongUsername | Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse::from(self))
    }
}

impl From<&AppError> for ErrorResponse {
    fn from(error: &AppError) -> Self {
        let status_code = error.status_code();
        Self::new(status_code.as_u16(), error.name(), error.to_string())
    }
}
