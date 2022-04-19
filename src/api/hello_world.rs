use actix_web::{web, Responder, get};
use crate::schemas::MessageSchema;

/// Hello world endpoint `</api/hello-world>`.
///
/// Return [`MessageSchema`] with `Hello World!`
#[get("/api/hello-world")]
pub async fn hello_world() -> impl Responder {
    web::Json(MessageSchema::new("Hello World!".to_owned()))
}
