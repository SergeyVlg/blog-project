use actix_web::{post, web, HttpResponse, Responder, Scope};
use chrono::Utc;
use tracing::info;
use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::error::{BlogError};
use crate::domain::user::UserWithToken;
use crate::presentation::dto::{HealthResponse, LoginRequest, RegisterRequest};

pub fn scope() -> Scope {
    web::scope("/api/auth")
        .route("/health", web::get().to(health))
        .service(register)
        .service(login)
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now(),
    })
}

#[post("/register")]
async fn register(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<RegisterRequest>,
) -> Result<impl Responder, BlogError> {
    let RegisterRequest {name, email, password} = payload.into_inner();
    let user_with_token = service
        .register(name, email, password)
        .await?;

    let UserWithToken { user, token } = user_with_token;

    info!(user_id = %user.id, email = %user.email, "user registered");

    Ok(HttpResponse::Created().json(serde_json::json!({
        "user": user,
        "token": token
    })))
}

#[post("/login")]
async fn login(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<LoginRequest>,
) -> Result<impl Responder, BlogError> {
    let user_with_token = service.login(&payload.email, &payload.password).await?;

    info!(email = %payload.email, "user logged in");

    let UserWithToken { user, token } = user_with_token;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": user,
        "token": token
    })))
}