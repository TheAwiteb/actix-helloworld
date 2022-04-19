use crate::{errors::AppError, schemas::querys::NameQuery, schemas::routes::MessageSchema};
use actix_web::{web, Responder};

/// Say hello to name utils.
///
///
/// Return [`MessageSchema`] with `Hello {name}` if there name
/// or `Hello Guest` if not
fn hello(query: NameQuery) -> Result<impl Responder, AppError> {
    if let Some(name) = query.name {
        // if name is long
        if name.chars().count().gt(&30) {
            let error: AppError = AppError::LongUsername;
            log::error!("{}", error.to_string());
            return Err(error);
        } else {
            return Ok(web::Json(MessageSchema::new(format!("Hello {}", name))));
        }
    }

    Ok(web::Json(MessageSchema::new("Hello Guest".to_owned())))
}

/// Say hello to name `</api/hello?name=<user name>>`.
///
/// Method: GET
///
/// Return [`MessageSchema`] with `Hello {name}` if there name
/// or `Hello Guest` if not
pub async fn hello_get(username: web::Query<NameQuery>) -> Result<impl Responder, AppError> {
    hello(username.0)
}

/// Say hello to name `</api/hello>`.
///
/// Method: POST
///
/// Return [`MessageSchema`] with `Hello {name}`
pub async fn hello_post(username: web::Json<NameQuery>) -> Result<impl Responder, AppError> {
    hello(username.0)
}

#[cfg(test)]
mod tests {
    use crate::api::configuration::config_routes;
    use crate::errors::{AppError, ErrorResponse};
    use crate::schemas::querys::NameQuery;
    use crate::schemas::routes::MessageSchema;
    use actix_web::{http::StatusCode, test, App};
    use serde_json::to_string;

    #[actix_web::test]
    async fn test_hello_get() {
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::get().uri("/api/hello").to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let body: MessageSchema = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&MessageSchema::new("Hello Guest".to_owned())).unwrap()
        );
    }

    #[actix_web::test]
    async fn test_hello_get_with_query() {
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::get()
            .uri("/api/hello?name=Awiteb")
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let body: MessageSchema = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&MessageSchema::new("Hello Awiteb".to_owned())).unwrap()
        );
    }

    #[actix_web::test]
    async fn test_hello_get_long_name_error() {
        let long_name = "a".repeat(31);
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::get()
            .uri(&format!("/api/hello?name={}", long_name))
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body: ErrorResponse = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&ErrorResponse::from(&AppError::LongUsername)).unwrap()
        );
    }

    #[actix_web::test]
    async fn test_hello_post() {
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::post()
            .uri("/api/hello")
            .set_json(NameQuery { name: None })
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let body: MessageSchema = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&MessageSchema::new("Hello Guest".to_owned())).unwrap()
        );
    }

    #[actix_web::test]
    async fn test_hello_post_with_payload() {
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::post()
            .uri("/api/hello")
            .set_json(NameQuery {
                name: Some("Awiteb".to_owned()),
            })
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let body: MessageSchema = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&MessageSchema::new("Hello Awiteb".to_owned())).unwrap()
        );
    }

    #[actix_web::test]
    async fn test_hello_post_long_name_error() {
        let long_name = "a".repeat(31);
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::post()
            .uri("/api/hello")
            .set_json(NameQuery {
                name: Some(long_name),
            })
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body: ErrorResponse = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&ErrorResponse::from(&AppError::LongUsername)).unwrap()
        );
    }
}
