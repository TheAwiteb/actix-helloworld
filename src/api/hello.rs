use crate::{errors::AppError, schemas::querys::NameQuery, schemas::routes::MessageSchema};
use actix_web::{web, Responder};

/// Say hello to name `</api/hello?name=<user name>>`.
///
/// Method: GET
///
/// Return [`MessageSchema`] with `Hello {name}` if there name
/// or `Hello Guest` if not
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

#[cfg(test)]
mod tests {
    use crate::api::configuration::config_routes;
    use crate::schemas::routes::MessageSchema;
    use actix_web::{http::StatusCode, test, App};
    use serde_json::{to_string};
    use crate::errors::{AppError, ErrorResponse};

    #[actix_web::test]
    async fn test_hello_get() {
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::get().uri("/api/hello").to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let body: MessageSchema = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&MessageSchema::new("Hello Guest".to_owned())).unwrap());
    }

    #[actix_web::test]
    async fn test_hello_get_with_query() {
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::get().uri("/api/hello?name=Awiteb").to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let body: MessageSchema = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&MessageSchema::new("Hello Awiteb".to_owned())).unwrap());
    }

    #[actix_web::test]
    async fn test_hello_get_long_name_error() {
        let long_name = "a".repeat(31);
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::get().uri(&format!("/api/hello?name={}", long_name)).to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body: ErrorResponse = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&ErrorResponse::from(&AppError::LongUsername)).unwrap());
    }
}