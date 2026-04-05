use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Scope};
use tracing::info;
use crate::application::blog_service::BlogService;
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::error::BlogError;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::dto::{CreatePostRequest, GetPostsRequest};

pub fn scope() -> Scope {
    web::scope("/api")
        .service(create_post)
        .service(list_accounts)
        .service(get_account)
        .service(deposit)
        .service(withdraw)
        .service(transfer)
        .service(exchange_rate)
}

fn ensure_owner(account: &AccountResponse, user: &AuthenticatedUser) -> Result<(), BlogError> {
    if account.owner_id != user.id {
        return Err(BlogError::Unauthorized)
    }

    Ok(())
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

#[get("/posts")]
async fn list_accounts(
    req: HttpRequest,
    user: AuthenticatedUser,
    blog: web::Data<BlogService<PostgresPostRepository>>,
    query: web::Query<GetPostsRequest>
) -> Result<HttpResponse, BlogError> {
    let GetPostsRequest { limit, offset} = query.into_inner();
    let posts = blog.list_posts(limit, offset).await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        count = response.len(),
        "accounts listed"
    );

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "posts": posts,
        "total": posts.len(),
        "limit": limit,
        "offset": offset,
    })))
}

#[get("/accounts/{id}")]
async fn get_account(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<PostgresAccountRepository>>,
    path: web::Path<i32>,
) -> Result<HttpResponse, BankError> {
    let account = bank.get_account(path.into_inner()).await?;
    let response = AccountResponse::from(account);
    ensure_owner(&response, &user)?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        account_id = response.id,
        "account fetched"
    );

    Ok(HttpResponse::Ok().json(response))
}

#[post("/accounts/{id}/deposit")]
async fn deposit(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<PostgresAccountRepository>>,
    path: web::Path<i32>,
    payload: web::Json<AmountRequest>,
) -> Result<HttpResponse, BankError> {
    let account_id = path.into_inner();
    let account = bank.get_account(account_id).await?;
    let response = AccountResponse::from(account.clone());
    ensure_owner(&response, &user)?;

    let account = bank.deposit(account_id, payload.amount).await?;
    let response = AccountResponse::from(account);

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        account_id = response.id,
        amount = payload.amount,
        "deposit completed"
    );

    Ok(HttpResponse::Ok().json(response))
}

#[post("/accounts/{id}/withdraw")]
async fn withdraw(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<PostgresAccountRepository>>,
    path: web::Path<i32>,
    payload: web::Json<AmountRequest>,
) -> Result<HttpResponse, BankError> {
    let account_id = path.into_inner();
    let account = bank.get_account(account_id).await?;
    let response = AccountResponse::from(account.clone());
    ensure_owner(&response, &user)?;

    let account = bank.withdraw(account_id, payload.amount).await?;
    let response = AccountResponse::from(account);

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        account_id = response.id,
        amount = payload.amount,
        "withdraw completed"
    );

    Ok(HttpResponse::Ok().json(response))
}

#[post("/transfers")]
async fn transfer(
    req: HttpRequest,
    user: AuthenticatedUser,
    bank: web::Data<BankService<PostgresAccountRepository>>,
    payload: web::Json<TransferRequest>,
) -> Result<HttpResponse, BankError> {
    let account = bank.get_account(payload.from).await?;
    if account.owner_id != user.id {
        return Err(BankError::Unauthorized);
    }

    bank.transfer(payload.from, payload.to, payload.amount).await?;

    info!(
        request_id = %request_id(&req),
        user_id = %user.id,
        from = payload.from,
        to = payload.to,
        amount = payload.amount,
        "transfer completed"
    );

    Ok(HttpResponse::Ok().json(serde_json::json!({ "status": "transferred" })))
}

#[get("/exchange/{from}/{to}")]
async fn exchange_rate(
    service: web::Data<ExchangeService>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, BankError> {
    let (from, to) = path.into_inner();
    let rate = service
        .get_rate(&from, &to)
        .await
        .map_err(|err| BankError::Internal(err.to_string()))?;

    Ok(HttpResponse::Ok().json(ExchangeResponse { from, to, rate }))
}