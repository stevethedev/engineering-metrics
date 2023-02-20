use actix_web::{get, post, HttpResponse, Responder};
use lib_authentication::{login, Login};
use lib_base64::Encode;

pub fn register(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(post_login);
}

#[post("/login")]
async fn post_login() -> impl Responder {
    match login_controller().await {
        None => HttpResponse::Unauthorized().finish(),
        Some(token) => HttpResponse::Ok().body(token),
    }
}

async fn login_controller() -> Option<String> {
    let token = login(&Login {
        username: "test",
        password: "test",
    });

    let token = match token {
        Ok(Some(token)) => token,
        Ok(None) => {
            return None;
        }
        Err(err) => {
            log::error!("Error while logging in: {}", err);
            return None;
        }
    };

    let token = match token.encode() {
        Ok(token) => token,
        Err(err) => {
            log::error!("Error while encoding token: {}", err);
            return None;
        }
    };

    Some(token)
}

#[post("/logout")]
async fn logout() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/register")]
async fn signup() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/whoami")]
async fn whoami() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
