use crate::errors::AppError;
use actix_web::{Responder, ResponseError};

// Not found response
pub async fn not_found() -> impl Responder {
    AppError::NotFound.error_response()
}

#[cfg(test)]
mod tests {
    use crate::api::configuration::config_routes;
    use crate::errors::{AppError, ErrorResponse};
    use actix_web::{http::StatusCode, test, App};
    use serde_json::to_string;

    #[actix_web::test]
    async fn test_not_found() {
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::get()
            .uri("/api/random/endpoint")
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::NOT_FOUND);

        let body: ErrorResponse = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&ErrorResponse::from(&AppError::NotFound)).unwrap()
        );
    }
}
