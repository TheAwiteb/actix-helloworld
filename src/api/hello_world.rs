use crate::schemas::routes::MessageSchema;
use actix_web::{web, Responder};

/// Hello world endpoint.
///
/// Endpoint: `/api/hello-world`
///
/// Method: GET
///
/// Request: Nothing
///
/// Response: [`MessageSchema`]
///
/// Return [`MessageSchema`] with `Hello World!`
pub async fn hello_world() -> impl Responder {
    web::Json(MessageSchema::new("Hello World!".to_owned()))
}

/// `/hello-world` initialize
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hello-world").route(web::get().to(hello_world)));
}

#[cfg(test)]
mod tests {
    use crate::api::configuration::config_routes;
    use crate::schemas::routes::MessageSchema;
    use actix_web::{http::StatusCode, test, App};
    use serde_json::to_string;

    #[actix_web::test]
    async fn test_hello_world_get() {
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::get()
            .uri("/api/hello-world")
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let body: MessageSchema = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&MessageSchema::new("Hello World!".to_owned())).unwrap()
        );
    }
}
