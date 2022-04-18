use super::querys::NameQuery;
use super::schemas::{MessageSchema, RouteSchema, RoutesSchema};
use actix_web::*;

/// Index endpoint `</api>`.
///
/// Return [`RoutesSchema`] with all endpoints
#[get("/api")]
pub async fn index() -> impl Responder {
    let routes = RoutesSchema::new(vec![
        RouteSchema::new("/api".to_owned(), "Explanation of all endpoints".to_owned()),
        RouteSchema::new("/api/hello-world".to_owned(), "Hello world endpoint".to_owned()),
        RouteSchema::new("/api/hello".to_owned(), "Say hello to user".to_owned()),
    ]);
    web::Json(routes)
}

/// Hello world endpoint `</api/hello-world>`.
///
/// Return [`MessageSchema`] with `Hello World!`
#[get("/api/hello-world")]
pub async fn hello_world() -> impl Responder {
    web::Json(MessageSchema::new("Hello World!".to_owned()))
}

/// Say hello to name `</api/hello?name=<user name>>`.
///
/// Return [`MessageSchema`] with `Hello {name}` if there name
/// or `Hello Guest` if not
#[get("/api/hello")]
pub async fn hello(username: web::Query<NameQuery>) -> impl Responder {
    web::Json(MessageSchema::new(format!(
        "Hello {}",
        username.name.as_ref().unwrap_or(&"Guest".to_owned())
    )))
}
