use actix_web::{get, post, web, HttpResponse, Responder};
use lib_authentication::{login, Login};
use lib_base64::Encode;
use lib_json_schema::schema::auth::{LoginRequest, LoginResponseSuccess};

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(post_login);
}

#[post("/login")]
async fn post_login(login_request: web::Json<LoginRequest>) -> impl Responder {
    match login_controller(&login_request).await {
        None => HttpResponse::Unauthorized().finish(),
        Some(response) => HttpResponse::Ok().json(response),
    }
}

async fn login_controller(login_request: &LoginRequest) -> Option<LoginResponseSuccess> {
    let token = login(&Login {
        username: &login_request.username,
        password: &login_request.password,
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

    Some(LoginResponseSuccess { token })
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
