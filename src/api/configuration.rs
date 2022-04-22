use crate::api;
use crate::errors::AppError;
use actix_web::web;
use actix_web::web::ServiceConfig;
use actix_web::{error, ResponseError};

/// Make App routes configuration
pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Set 404 service
            .default_service(web::route().to(api::not_found))
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    AppError::BadRequest(format!("{}", err)).error_response(),
                )
                .into()
            }))
            .service(web::resource("").route(web::get().to(api::index)))
            .service(web::resource("/hello-world").route(web::get().to(api::hello_world)))
            .service(
                web::resource("/hello")
                    .route(web::get().to(api::hello_get))
                    .route(web::post().to(api::hello_post)),
            ),
    );
}
