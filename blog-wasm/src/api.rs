use blog_client::GetPostsResponse;
use gloo_net::http::Request;

const POSTS_URL: &str = "http://127.0.0.1:8080/api/public/posts?limit=10&offset=0";

pub async fn fetch_posts() -> Result<GetPostsResponse, String> {
    Request::get(POSTS_URL)
        .send()
        .await
        .map_err(|error| format!("Не удалось выполнить запрос списка постов: {error}"))?
        .json::<GetPostsResponse>()
        .await
        .map_err(|error| format!("Не удалось разобрать ответ сервера: {error}"))
}

