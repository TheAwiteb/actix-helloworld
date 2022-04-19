use crate::{
    errors::{AppError, ErrorResponse},
    schemas::routes::{ParamsSchema, RouteSchema, RoutesSchema},
};
use actix_web::{web, Responder};

fn routes() -> RoutesSchema {
    RoutesSchema::new(vec![
        RouteSchema::new(
            "/api".to_owned(),
            "Explanation of all endpoints".to_owned(),
            None,
            None,
        ),
        RouteSchema::new(
            "/api/hello-world".to_owned(),
            "Hello world endpoint".to_owned(),
            None,
            None,
        ),
        RouteSchema::new(
            "/api/hello".to_owned(),
            "Say hello to user".to_owned(),
            Some(vec![ParamsSchema::new(
                "name".to_owned(),
                "Name of user".to_owned(),
                true,
            )]),
            Some(vec![ErrorResponse::from(&AppError::LongUsername)]),
        ),
    ])
}

/// Index endpoint `</api>`.
///
/// Return [`RoutesSchema`] with all endpoints
pub async fn index() -> impl Responder {
    let routes = routes();
    web::Json(routes)
}

#[cfg(test)]
mod tests {
    use crate::api::configuration::config_routes;
    use crate::schemas::routes::RoutesSchema;
    use actix_web::{http::StatusCode, test, App};
    use serde_json::{to_string};

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().configure(config_routes)).await;
        let req = test::TestRequest::get().uri("/api").to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::OK);

        let body: RoutesSchema = test::read_body_json(res).await;

        assert_eq!(
            to_string(&body).unwrap(),
            to_string(&super::routes()).unwrap());
    }
}
