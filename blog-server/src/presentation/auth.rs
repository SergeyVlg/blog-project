use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use actix_web::dev::Payload;
use uuid::Uuid;
use futures_util::future::{ready, Ready};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub name: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.extensions().get::<AuthenticatedUser>() {
            Some(user) => ready(Ok(user.clone())),
            None => ready(Err(ErrorUnauthorized("missing authenticated user"))),
        }
    }
}