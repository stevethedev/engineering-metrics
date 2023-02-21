use crate::controllers::auth::{login, logout, whoami};
use crate::middleware::bearer_token::RequestToken;
use actix_web::{get, post, web, HttpResponse, Responder};
use lib_json_schema::schema::auth::LoginRequest;

/// Registers the routes for the authentication module.
pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(post_login)
        .service(get_whoami)
        .service(get_logout)
        .service(post_register);
}

/// Authenticates a user and returns a token.
///
/// # Arguments
///
/// - `provider` - The authentication provider.
/// - `login_request` - The login request.
///
/// # Returns
///
/// - HTTP 200 with the token if the login was successful.
/// - HTTP 401 if the login failed.
#[post("/login")]
async fn post_login(
    provider: web::Data<lib_authentication::Provider>,
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    match login(&provider, &login_request).await {
        None => HttpResponse::Unauthorized().finish(),
        Some(response) => HttpResponse::Ok().json(response),
    }
}

#[get("/logout")]
async fn get_logout(
    provider: web::Data<lib_authentication::Provider>,
    bearer_token: RequestToken,
) -> impl Responder {
    let Some(token) = bearer_token.as_ref() else { return HttpResponse::Unauthorized().finish() };
    match logout(&provider, token).await {
        Err(_) => HttpResponse::Unauthorized().finish(),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}

#[post("/register")]
async fn post_register() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// Returns the user information for the given token.
///
/// # Arguments
///
/// - `provider` - The authentication provider.
/// - `bearer_token` - The bearer token.
///
/// # Returns
///
/// - HTTP 200 with the user information if the token is valid.
/// - HTTP 401 if the token is invalid.
#[get("/whoami")]
async fn get_whoami(
    provider: web::Data<lib_authentication::Provider>,
    bearer_token: RequestToken,
) -> impl Responder {
    let result = whoami(&provider, bearer_token.as_ref()).await;

    match result {
        None => HttpResponse::Unauthorized().finish(),
        Some(data) => HttpResponse::Ok().json(data),
    }
}
