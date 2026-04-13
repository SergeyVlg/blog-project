use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, Scope};
use chrono::Utc;
use tracing::info;
use uuid::Uuid;
use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::error::{BlogError};
use crate::domain::user::UserWithToken;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::dto::{GetPostsRequest, HealthResponse, LoginRequest, RegisterRequest};

pub fn scope() -> Scope {
    web::scope("/auth")
        .service(register)
        .service(login);
    web::scope("")
        .route("/health", web::get().to(health))
        .service(get_post)
        .service(list_posts)
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
    let user_with_token = service.login(&payload.name, &payload.password).await?;

    info!(name = %payload.name, "user logged in");

    let UserWithToken { user, token } = user_with_token;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "user": user,
        "token": token
    })))
}

#[get("/posts/{id}")]
async fn get_post(
    blog: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, BlogError> {
    let post_id = path.into_inner();
    let post = blog.get_post(post_id).await?;

    info!(
        post_id = %post_id,
        "post has gotten"
    );

    Ok(HttpResponse::Ok().json(post))
}

#[get("/posts")]
async fn list_posts(
    blog: web::Data<BlogService<PostgresPostRepository>>,
    query: web::Query<GetPostsRequest>
) -> Result<HttpResponse, BlogError> {
    let GetPostsRequest { limit, offset} = query.into_inner();
    let posts = blog.list_posts(limit, offset).await?;

    info!(
        limit = %limit,
        offset = %offset,
        "list posts"
    );

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "posts": posts,
        "total": posts.len(),
        "limit": limit,
        "offset": offset,
    })))
}