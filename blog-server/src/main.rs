mod presentation;
mod application;
mod domain;
mod data;
mod infrastructure;

use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Context;
use infrastructure::{logging::init_logging, config::Config, database};
use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::jwt::JwtKeys;
use crate::presentation::http_handlers;
use crate::presentation::middleware::JwtAuthMiddleware;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().context("Env file error")?;
    init_logging();

    tracing::info!("Starting server...");

    let config = Config::from_env().context("Invalid config")?;
    let pool = database::create_pool(&config.database_url).await.context("failed to connect to database")?;

    database::run_migrations(&pool).await.context("migrations failed")?;

    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("→ listening on http://{}", addr);

    //что лучше - Arc на репозиторий или сервис?
    let user_repo = PostgresUserRepository::new(pool.clone());
    let post_repo = PostgresPostRepository::new(pool.clone());

    let auth_service = Arc::new(AuthService::new(user_repo, JwtKeys::new(config.jwt_secret.clone())));
    let blog_service = Arc::new(BlogService::new(post_repo));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.cors_origin)
            .allowed_methods(vec!["GET","POST","PUT","DELETE","OPTIONS"])
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(blog_service.clone()))
            .service(
                web::scope("/api")
                    .service(http_handlers::public::scope())
                    .service(
                        web::scope("")
                            .wrap(JwtAuthMiddleware::new(auth_service.keys().clone()))
                            .service(http_handlers::protected::scope()),
                    ),
            )
    })
        .bind(addr)?
        .run()
        .await
        .context("server error")?;

    Ok(())
}
