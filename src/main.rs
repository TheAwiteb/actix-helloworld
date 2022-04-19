mod api;
mod errors;
mod schemas;
mod utils;

use actix_web::web::scope;
use actix_web::*;
use api::{hello, hello_world, index};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();
    let host: String = std::env::var("ACTIX_HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port: String = std::env::var("ACTIX_PORT").unwrap_or_else(|_| "8080".to_owned());

    log::info!("Running server on http://{}:{}", host, port);
    HttpServer::new(|| {
        App::new()
            // This middleware to logging in terminal
            .wrap(middleware::Logger::default())
            // This middleware to remove end slash from url if its there
            .wrap(middleware::NormalizePath::trim())
            .service(
                scope("/api")
                    .service(index)
                    .service(hello_world)
                    .service(hello),
            )
    })
    .bind((host.as_str(), port.parse().unwrap_or(8080)))?
    .run()
    .await
}
