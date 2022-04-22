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
            // Set json payload error
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    AppError::BadRequest(format!("{}", err)).error_response(),
                )
                .into()
            }))
            // Index initialize
            .configure(api::index::init)
            // hello initialize
            .configure(api::hello::init)
            // hello-world initialize
            .configure(api::hello_world::init),
    );
}
