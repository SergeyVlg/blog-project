use actix_web::{post, web, HttpResponse, Responder, Scope};
use chrono::Utc;
use tracing::info;
use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::error::BlogError;
use crate::presentation::dto::{LoginRequest, RegisterRequest};

pub fn scope() -> Scope {
    web::scope("")
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

#[post("/auth/register")]
async fn register(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<RegisterRequest>,
) -> Result<impl Responder, BlogError> {
    let user_with_token = service
        .register(payload.name, payload.email, payload.password)
        .await?;

    let user = user_with_token.user;
    let token = user_with_token.token;

    info!(user_id = %user.id, email = %user.email, "user registered");

    Ok(HttpResponse::Created().json(serde_json::json!({
        "user": user,
        "token": token
    })))
}

#[post("/auth/login")]
async fn login(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<LoginRequest>,
) -> Result<impl Responder, BlogError> {
    let jwt = service.login(&payload.email, &payload.password).await?;
    info!(email = %payload.email, "user logged in");
    Ok(HttpResponse::Ok().json(TokenResponse { access_token: jwt }))
}