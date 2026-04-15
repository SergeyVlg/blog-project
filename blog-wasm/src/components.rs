use blog_client::Post;
use dioxus::prelude::*;

use crate::api::fetch_posts;

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
                }

                match &*posts_resource.read() {
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

