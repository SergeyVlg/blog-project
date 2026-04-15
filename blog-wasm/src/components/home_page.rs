use dioxus::prelude::*;

use crate::api::fetch_posts;

use super::post_card::PostCard;
use super::registration_modal::RegistrationModal;

#[component]
pub fn HomePage() -> Element {
    let posts_resource = use_resource(fetch_posts);
    let mut show_registration = use_signal(|| false);
    let mut registration_success = use_signal(|| Option::<String>::None);

    let is_registration_open = show_registration();
    let registration_action_label = if is_registration_open {
        "Скрыть регистрацию"
    } else {
        "Регистрация"
    };

    rsx! {
        main {
            section {
                class: "posts-page",

                header {
                    class: "posts-page__header",

                    div {
                        class: "posts-page__header-title",
                        h1 { "Посты" }
                    }

                    nav {
                        class: "posts-page__auth-links",

                        span {
                            class: "posts-page__auth-link posts-page__auth-link_disabled",
                            "Вход"
                        }

                        button {
                            class: "posts-page__auth-link",
                            r#type: "button",
                            onclick: move |_| {
                                show_registration.set(!show_registration());
                            },
                            "{registration_action_label}"
                        }
                    }
                }

                match &*registration_success.read() {
                    Some(message) => rsx! {
                        p {
                            class: "posts-page__notice posts-page__notice_success",
                            "{message}"
                        }
                    },
                    None => rsx! {},
                }

                if is_registration_open {
                    RegistrationModal {
                        on_close: move |_| {
                            show_registration.set(false);
                        },
                        on_success: move |message: String| {
                            registration_success.set(Some(message));
                            show_registration.set(false);
                        }
                    }
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
                            },
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



