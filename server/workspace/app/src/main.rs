#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

use actix_web::{middleware as aw_middleware, web, App, HttpServer};

mod controllers;
mod middleware;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let logger_format = "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %D";

    let auth_token_repo = lib_authentication::TokenRepo::memory();
    let refresh_token_repo = lib_authentication::TokenRepo::memory();
    let user_repo = lib_authentication::UserRepo::memory();
    let auth_provider =
        lib_authentication::Provider::new(auth_token_repo, refresh_token_repo, user_repo);
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
