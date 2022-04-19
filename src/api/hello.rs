use crate::{errors::AppError, schemas::querys::NameQuery, schemas::routes::MessageSchema};
use actix_web::{get, web, Responder};

/// Say hello to name `</api/hello?name=<user name>>`.
///
/// Return [`MessageSchema`] with `Hello {name}` if there name
/// or `Hello Guest` if not
#[get("/hello")]
pub async fn hello(username: web::Query<NameQuery>) -> Result<impl Responder, AppError> {
    if username.name.is_some() && username.name.as_ref().unwrap().chars().count() > 30 {
        let error: AppError = AppError::LongUsername;
        log::error!("{}", error.to_string());
        Err(error)
    } else {
        Ok(web::Json(MessageSchema::new(format!(
            "Hello {}",
            username.name.as_ref().unwrap_or(&"Guest".to_owned())
        ))))
    }
}
