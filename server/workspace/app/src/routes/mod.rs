use actix_web::web;

mod auth;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").configure(auth::register));
}
