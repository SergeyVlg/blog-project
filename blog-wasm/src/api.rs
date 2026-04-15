use blog_client::dto::{RegisterRequest, UserWithToken};
use blog_client::GetPostsResponse;
use gloo_net::http::Request;

const POSTS_URL: &str = "http://127.0.0.1:8080/api/public/posts?limit=10&offset=0";
const REGISTER_URL: &str = "http://127.0.0.1:8080/api/public/register";

pub async fn fetch_posts() -> Result<GetPostsResponse, String> {
    Request::get(POSTS_URL)
        .send()
        .await
        .map_err(|error| format!("Не удалось выполнить запрос списка постов: {error}"))?
        .json::<GetPostsResponse>()
        .await
        .map_err(|error| format!("Не удалось разобрать ответ сервера: {error}"))
}

pub async fn register_user(name: String, email: String, password: String) -> Result<String, String> {
    let response = Request::post(REGISTER_URL)
        .json(&RegisterRequest { name, email, password })
        .map_err(|error| format!("Не удалось подготовить запрос регистрации: {error}"))?
        .send()
        .await
        .map_err(|error| format!("Не удалось выполнить запрос регистрации: {error}"))?;

    if !response.ok() {
        let status = response.status();
        let details = response.text().await.unwrap_or_default();
        let message = if details.trim().is_empty() {
            format!("код ответа {status}")
        } else {
            details
        };

        return Err(format!("Регистрация не выполнена: {message}"));
    }

    let payload = response
        .json::<UserWithToken>()
        .await
        .map_err(|error| format!("Не удалось разобрать ответ сервера: {error}"))?;

    Ok(format!("Пользователь «{}» успешно зарегистрирован.", payload.user.name))
}

