use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::env;
use std::future::{ready, Ready};

pub fn auth_middleware() -> HttpAuthentication<
    BearerAuth,
    impl Fn(ServiceRequest, BearerAuth) -> Ready<Result<ServiceRequest, (Error, ServiceRequest)>>,
> {
    HttpAuthentication::bearer(|req: ServiceRequest, credentials: BearerAuth| {
        ready({
            match env::var("API_TOKEN") {
                Ok(token) if credentials.token() == token => Ok(req),
                _ => Err((ErrorUnauthorized("Invalid token"), req)),
            }
        })
    })
}
