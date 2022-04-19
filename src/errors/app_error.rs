use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_new::new;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The username should be smaller than 30 character")]
    LongUsername,
}
impl AppError {
    pub fn name(&self) -> String {
        match self {
            Self::LongUsername => "LongUsername".to_owned(),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::LongUsername => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse::from(self))
    }
}

impl From<&AppError> for ErrorResponse {
    fn from(error: &AppError) -> Self {
        let status_code = error.status_code();
        ErrorResponse::new(status_code.as_u16(), error.name(), error.to_string())
    }
}

#[derive(Serialize, Deserialize, new)]
pub struct ErrorResponse {
    #[new(value = "false")]
    pub status: bool,
    code: u16,
    error: String,
    message: String,
}
