#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

mod routes;

use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let logger_format = "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %D";

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::new(logger_format))
            .configure(routes::register)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
