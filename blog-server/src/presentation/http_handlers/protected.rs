use actix_web::{delete, post, put, web, HttpResponse, Scope};
use tracing::info;
use uuid::Uuid;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::error::BlogError;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::dto::{CreatePostRequest, UpdatePostRequest};

pub fn scope() -> Scope {
    web::scope("")
        .service(create_post)
        .service(update_post)
        .service(delete_post)
}

#[post("/posts")]
async fn create_post(
    user: AuthenticatedUser,
    blog: web::Data<BlogService<PostgresPostRepository>>,
    payload: web::Json<CreatePostRequest>,
) -> Result<HttpResponse, BlogError> {
    let CreatePostRequest { title, content } = payload.into_inner();
    let post = blog
        .create_post(user.id, title, content)
        .await?;

    info!(
        post_id = %post.id,
        author = %user.name,
        "post created"
    );

    Ok(HttpResponse::Created().json(post))
}

#[put("/posts/{id}")]
async fn update_post(
    user: AuthenticatedUser,
    blog: web::Data<BlogService<PostgresPostRepository>>,
    payload: web::Json<UpdatePostRequest>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, BlogError> {
    let UpdatePostRequest { title, content } = payload.into_inner();
    let post_id = path.into_inner();
    let post = blog
        .update_post(user.id, post_id, title, content)
        .await?;

    info!(
        post_id = %post_id,
        author_id = %user.id,
        author = %user.name,
        "post updated"
    );

    Ok(HttpResponse::Created().json(post))
}

#[delete("/posts/{id}")]
async fn delete_post(
    user: AuthenticatedUser,
    blog: web::Data<BlogService<PostgresPostRepository>>,
    path: web::Path<Uuid>,
) -> Result<HttpResponse, BlogError> {
    let post_id = path.into_inner();
    blog.delete_post(user.id, post_id).await?;

    info!(
        post_id = %post_id,
        author_id = %user.id,
        author = %user.name,
        "post deleted"
    );

    Ok(HttpResponse::NoContent().finish())
}