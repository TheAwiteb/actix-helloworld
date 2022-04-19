use crate::{
    errors::{AppError, ErrorResponse},
    schemas::routes::{ParamsSchema, RouteSchema, RoutesSchema},
};
use actix_web::{get, web, Responder};

/// Index endpoint `</api>`.
///
/// Return [`RoutesSchema`] with all endpoints
#[get("")]
pub async fn index() -> impl Responder {
    let routes = RoutesSchema::new(vec![
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
    ]);
    web::Json(routes)
}
