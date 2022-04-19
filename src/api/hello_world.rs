use crate::schemas::routes::MessageSchema;
use actix_web::{get, web, Responder};

/// Hello world endpoint `</api/hello-world>`.
///
/// Return [`MessageSchema`] with `Hello World!`
#[get("/hello-world")]
pub async fn hello_world() -> impl Responder {
    web::Json(MessageSchema::new("Hello World!".to_owned()))
}
