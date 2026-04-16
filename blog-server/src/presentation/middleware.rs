use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::jwt::JwtKeys;
use crate::presentation::auth::jwt_validator;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, Error, HttpMessage};
use futures_util::future::{ready, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};
use actix_web::http::StatusCode;
use tracing::warn;

pub struct JwtAuthMiddleware {
    keys: JwtKeys,
}

impl JwtAuthMiddleware {
    pub fn new(keys: JwtKeys) -> Self {
        Self { keys }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthService {
            service: Rc::new(service),
            keys: self.keys.clone(),
        }))
    }
}

pub struct JwtAuthService<S> {
    service: Rc<S>,
    keys: JwtKeys,
}

impl<S, B> Service<ServiceRequest> for JwtAuthService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let keys = self.keys.clone();
        let service = Rc::clone(&self.service);

        let auth_service = req
            .app_data::<web::Data<Arc<AuthService<PostgresUserRepository>>>>()
            .cloned();

        let auth_header = req
            .headers()
            .get(actix_web::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(|value| value.to_string());


        Box::pin(async move {
            let auth_service = auth_service
                .ok_or_else(|| actix_web::error::ErrorInternalServerError("AuthService missing"))?;

            let header = auth_header
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("missing authorization header"))?;
            let token = header
                .strip_prefix("Bearer ")
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("invalid authorization header"))?;

            let user = jwt_validator(token, &keys, auth_service.get_ref()).await?;

            req.extensions_mut().insert(user);
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

pub struct JwtAuthErrorLogger;

impl JwtAuthErrorLogger {
    pub fn new() -> Self {
        Self
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuthErrorLogger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthErrorLoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthErrorLoggerMiddleware { service }))
    }
}

pub struct JwtAuthErrorLoggerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtAuthErrorLoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_owned();

        let fut = self.service.call(req);

        Box::pin(async move {
            match fut.await {
                Ok(res) => {
                    let status = res.status();

                    if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
                        warn!(
                            method = %method,
                            path = %path,
                            status = %status,
                            "protected request finished with auth error status"
                        );
                    }

                    Ok(res)
                }
                Err(err) => {
                    let status = err.as_response_error().status_code();

                    warn!(
                        method = %method,
                        path = %path,
                        status = %status,
                        error = %err,
                        "protected request failed"
                    );

                    Err(err)
                }
            }
        })
    }
}
