use actix_web::{get, post, web, HttpResponse, Responder};

use lib_authentication::Provider;
use lib_json_schema::schema::auth::{LoginRequest, RefreshRequest, RegisterRequest};

use crate::controllers::auth::{login, logout, refresh, register as register_user, whoami};
use crate::middleware::bearer_token::RequestToken;

/// Registers the routes for the authentication module.
pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(post_login)
        .service(get_whoami)
        .service(get_logout)
        .service(post_refresh)
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
    provider: web::Data<Provider>,
    login_request: web::Json<LoginRequest>,
) -> impl Responder {
    match login(&provider, &login_request).await {
        None => HttpResponse::Unauthorized().finish(),
        Some(response) => HttpResponse::Ok().json(response),
    }
}

/// Refreshes a token.
///
/// # Arguments
///
/// - `provider` - The authentication provider.
/// - `refresh_request` - The refresh request.
///
/// # Returns
///
/// - HTTP 200 with the token if the refresh was successful.
/// - HTTP 401 if the refresh failed.
#[post("/refresh")]
async fn post_refresh(
    provider: web::Data<Provider>,
    refresh_request: web::Json<RefreshRequest>,
) -> impl Responder {
    match refresh(provider.as_ref(), &refresh_request).await {
        None => HttpResponse::Unauthorized().finish(),
        Some(response) => HttpResponse::Ok().json(response),
    }
}

/// De-authenticates a user with the given token.
///
/// # Arguments
///
/// - `provider` - The authentication provider.
/// - `bearer_token` - The bearer token.
///
/// # Returns
///
/// - HTTP 200 if the logout was successful.
/// - HTTP 401 if the logout failed.
#[get("/logout")]
async fn get_logout(provider: web::Data<Provider>, bearer_token: RequestToken) -> impl Responder {
    let Some(token) = bearer_token.as_ref() else { return HttpResponse::Unauthorized().finish() };
    match logout(provider.as_ref(), token).await {
        Err(_) => HttpResponse::Unauthorized().finish(),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}

/// Registers a new user.
///
/// # Arguments
///
/// - `provider` - The authentication provider.
/// - `register_request` - The register request.
///
/// # Returns
///
/// - HTTP 201 if the registration was successful.
/// - HTTP 401 if the registration failed.
#[post("/register")]
async fn post_register(
    provider: web::Data<Provider>,
    register_credentials: web::Json<RegisterRequest>,
) -> impl Responder {
    match register_user(provider.as_ref(), &register_credentials).await {
        Err(_) => HttpResponse::Unauthorized().finish(),
        Ok(_) => HttpResponse::Created().finish(),
    }
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
async fn get_whoami(provider: web::Data<Provider>, bearer_token: RequestToken) -> impl Responder {
    let result = whoami(provider.as_ref(), bearer_token.as_ref()).await;

    match result {
        None => HttpResponse::Unauthorized().finish(),
        Some(data) => HttpResponse::Ok().json(data),
    }
}
