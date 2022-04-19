use crate::api;
use actix_web::web;
use actix_web::web::ServiceConfig;

/// Make App routes configuration
pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("").route(web::get().to(api::index)))
            .service(web::resource("/hello-world").route(web::get().to(api::hello_world)))
            .service(web::resource("/hello").route(web::get().to(api::hello))),
    );
}