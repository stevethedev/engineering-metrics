use actix_web::body::MessageBody;
use actix_web::web::ReqData;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};

pub use lib_authentication::AuthToken;
use lib_base64::decode;

pub type RequestToken = ReqData<Option<AuthToken>>;

/// A middleware that extracts the token from the `Authorization` header.
#[derive(Default)]
pub struct BearerToken;

impl<S, B> Transform<S, ServiceRequest> for BearerToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = Middleware<S>;
    type InitError = ();
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(Middleware { service })
    }
}

pub struct Middleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for Middleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    B: MessageBody,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .and_then(|token| decode(token).ok())
            .map(AuthToken::from);

        req.extensions_mut().insert(token);

        self.service.call(req)
    }
}
