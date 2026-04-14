mod presentation;
mod application;
mod domain;
mod data;
mod infrastructure;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Context;
use futures_util::{pin_mut, select, FutureExt };
use sqlx::PgPool;
use infrastructure::{logging::init_logging, config::Config, database};
use tonic::transport::Server;
use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::data::user_repository::PostgresUserRepository;
use crate::infrastructure::jwt::JwtKeys;
use crate::presentation::grpc_service::BlogGrpcService;
use crate::presentation::http_handlers;
use crate::presentation::middleware::JwtAuthMiddleware;
use crate::presentation::proto::blog::blog_service_server::BlogServiceServer;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    init_logging();

    tracing::info!("Starting server...");

    let config = Config::from_env().context("Invalid config")?;
    let pool = database::create_pool(&config.database_url).await.context("failed to connect to database")?;

    database::run_migrations(&pool).await.context("migrations failed")?;

    let user_repo = PostgresUserRepository::new(pool.clone());
    let post_repo = PostgresPostRepository::new(pool.clone());

    let auth_service = Arc::new(AuthService::new(user_repo, JwtKeys::new(config.jwt_secret.clone())));
    let blog_service = Arc::new(BlogService::new(post_repo));

    let grpc_ip: Ipv4Addr = config.host.parse()?;
    let grpc_addr = SocketAddr::new(IpAddr::V4(grpc_ip), config.grpc_port);

    let http = http_server(config, pool, auth_service.clone(), blog_service.clone()).fuse();
    let grpc = grpc_server(grpc_addr, auth_service.clone(), blog_service.clone()).fuse();
    pin_mut!(http, grpc);

    let result = select! {
        res = http => res,
        res = grpc => res,
    };

    result?;

    Ok(())
}

async fn http_server(config: Config,
                     pool: PgPool,
                     auth_service: Arc<AuthService<PostgresUserRepository>>,
                     blog_service: Arc<BlogService<PostgresPostRepository>>) -> anyhow::Result<()> {
    let addr = format!("{}:{}", config.host, config.port);

    tracing::info!("→ listening on http://{}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            //.allowed_origin(&config.cors_origin)
            .allow_any_origin()
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
                web::scope("/api/public")
                    .service(http_handlers::public::scope())
            )
            .service(
                web::scope("/api/protected")
                    .wrap(JwtAuthMiddleware::new(auth_service.keys().clone()))
                    .service(http_handlers::protected::scope()),
            )
    })
        .bind(addr)?
        .run()
        .await
        .context("http server error")?;

    Ok(())
}

async fn grpc_server(addr: SocketAddr,
                     auth_service: Arc<AuthService<PostgresUserRepository>>,
                     blog_service: Arc<BlogService<PostgresPostRepository>>) -> anyhow::Result<()> {


    let service = BlogGrpcService {
        auth_service,
        blog_service
    };

    Server::builder()
        .add_service(BlogServiceServer::new(service))
        .serve(addr)
        .await
        .context("grpc server error")?;

    Ok(())
}
