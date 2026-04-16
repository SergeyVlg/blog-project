use blog_client::dto::{LoginRequest, RegisterRequest, UserWithToken};
use blog_client::GetPostsResponse;
use gloo_net::http::Request;

use crate::storage;

const API_BASE_URL: &str = "http://127.0.0.1:8080/api";

const POSTS_PATH: &str = "/public/posts?limit=10&offset=0";
const REGISTER_PATH: &str = "/public/register";
const LOGIN_PATH: &str = "/public/login";

pub async fn fetch_posts() -> Result<GetPostsResponse, String> {
    Request::get(&format!("{API_BASE_URL}{POSTS_PATH}"))
        .send()
        .await
        .map_err(|error| format!("Не удалось выполнить запрос списка постов: {error}"))?
        .json::<GetPostsResponse>()
        .await
        .map_err(|error| format!("Не удалось разобрать ответ сервера: {error}"))
}

pub async fn register_user(name: String, email: String, password: String) -> Result<String, String> {
    let response = Request::post(&format!("{API_BASE_URL}{REGISTER_PATH}"))
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

    storage::save_token(payload.token.as_str());

    Ok(format!("Пользователь «{}» успешно зарегистрирован.", payload.user.name))
}

pub async fn login_user(name: String, password: String) -> Result<(), String> {
    let response = Request::post(&format!("{API_BASE_URL}{LOGIN_PATH}"))
        .json(&LoginRequest { name, password })
        .map_err(|error| format!("Не удалось подготовить запрос входа: {error}"))?
        .send()
        .await
        .map_err(|error| format!("Не удалось выполнить запрос входа: {error}"))?;

    if !response.ok() {
        let status = response.status();
        let details = response.text().await.unwrap_or_default();
        let message = if details.trim().is_empty() {
            format!("код ответа {status}")
        } else {
            details
        };

        return Err(message);
    }

    let payload = response
        .json::<UserWithToken>()
        .await
        .map_err(|error| format!("Не удалось разобрать ответ сервера: {error}"))?;

    storage::save_token(payload.token.as_str());

    Ok(())
}

