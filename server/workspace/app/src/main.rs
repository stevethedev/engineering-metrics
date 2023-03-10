#![deny(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic
)]

use actix_web::{middleware as aw_middleware, web, App, HttpServer};

use lib_environment::{EnvironmentVariable, RedisCacheConnectionString};

mod controllers;
mod database;
mod middleware;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let logger_format = "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %D";

    let Ok(db_connection) = database::open().await else {
        log::error!("Failed to open database connection");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to open database connection",
        ));
    };

    let redis_url = RedisCacheConnectionString::get();
    let Ok(cache_controller) = lib_cache::Controller::open(&redis_url) else {
        log::error!("Failed to open cache connection");
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to open cache connection",
        ));
    };

    let user_repo = lib_authentication::UserRepo::database(db_connection.clone());
    let auth_token_repo =
        lib_authentication::TokenRepo::cache(cache_controller.clone(), "a".to_string());
    let refresh_token_repo =
        lib_authentication::TokenRepo::cache(cache_controller.clone(), "r".to_string());

    // let auth_token_repo = lib_authentication::TokenRepo::memory();
    // let refresh_token_repo = lib_authentication::TokenRepo::memory();
    // let user_repo = lib_authentication::UserRepo::memory();
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
    .await?;

    Ok(())
}
