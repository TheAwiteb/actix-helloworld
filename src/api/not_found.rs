use crate::errors::AppError;
use actix_web::{Responder, ResponseError};

// Not found response
pub async fn not_found() -> impl Responder {
    AppError::NotFound.error_response()
}
