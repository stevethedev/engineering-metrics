#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

mod controllers;
mod middleware;
mod routes;

use actix_web::{middleware as aw_middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let logger_format = "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %D";

    let token_repo = lib_authentication::TokenRepo::memory();
    let user_repo = lib_authentication::UserRepo::memory();
    let auth_provider = lib_authentication::Provider::new(token_repo, user_repo);
    let auth_provider = web::Data::new(auth_provider);

    HttpServer::new(move || {
        App::new()
            .app_data(auth_provider.clone())
            .wrap(aw_middleware::Logger::new(logger_format))
            .wrap(middleware::BearerToken)
            .configure(routes::register)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
