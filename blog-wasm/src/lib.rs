use blog_client::{GetPostsResponse, Post};
use dioxus::prelude::*;
use gloo_net::http::Request;

const POSTS_URL: &str = "http://127.0.0.1:8080/api/public/posts?limit=10&offset=0";

async fn fetch_posts() -> Result<GetPostsResponse, String> {
    Request::get(POSTS_URL)
        .send()
        .await
        .map_err(|error| format!("Не удалось выполнить запрос списка постов: {error}"))?
        .json::<GetPostsResponse>()
        .await
        .map_err(|error| format!("Не удалось разобрать ответ сервера: {error}"))
}

#[component]
pub fn App() -> Element {
    let posts_resource = use_resource(fetch_posts);

    rsx! {
        main {
            section {
                class: "posts-page",

                header {
                    class: "posts-page__header",
                    h1 { "Посты" }
                    p { "Стартовая страница загружает список постов по HTTP GET запросу." }
                }

                match &*posts_resource.read_unchecked() {
                    Some(Ok(response)) if response.posts.is_empty() => rsx! {
                        p {
                            class: "posts-page__state",
                            "Постов пока нет."
                        }
                    },
                    Some(Ok(response)) => rsx! {
                        div {
                            class: "posts-list",
                            for post in response.posts.iter() {
                                PostCard { key: "{post.id}", post: post.clone() }
                            }
                        }
                    },
                    Some(Err(error)) => rsx! {
                        p {
                            class: "posts-page__state posts-page__state_error",
                            "Ошибка загрузки постов: {error}"
                        }
                    },
                    None => rsx! {
                        p {
                            class: "posts-page__state",
                            "Загрузка постов..."
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn PostCard(post: Post) -> Element {
    rsx! {
        article {
            class: "post-card",

            h2 {
                class: "post-card__title",
                "{post.title}"
            }

            p {
                class: "post-card__content",
                "{post.content}"
            }
        }
    }
}
