use dioxus::prelude::*;
use blog_client::Post;

use crate::api::fetch_posts;
use crate::storage;

use super::login_modal::LoginModal;
use super::post_modal::PostModal;
use super::post_card::PostCard;
use super::registration_modal::RegistrationModal;

#[component]
pub fn HomePage() -> Element {
    let mut posts_reload_key = use_signal(|| 0_u64);
    let posts_resource = use_resource(move || {
        let _ = posts_reload_key();
        fetch_posts()
    });
    let mut show_login = use_signal(|| false);
    let mut show_registration = use_signal(|| false);
    let mut show_new_post = use_signal(|| false);
    let mut editing_post = use_signal(|| Option::<Post>::None);
    let mut registration_success = use_signal(|| Option::<String>::None);
    let mut auth = use_signal(storage::Auth::new);

    use_context_provider(|| auth);

    let is_login_open = show_login();
    let is_registration_open = show_registration();
    let is_new_post_open = show_new_post();
    let editing_post_value = editing_post();
    let is_edit_post_open = editing_post_value.is_some();
    let modal_key = editing_post_value
        .as_ref()
        .map(|post| format!("edit-{}", post.id))
        .unwrap_or_else(|| "create".to_string());

    let token = auth().token.clone();
    let is_authenticated = auth().is_authenticated();

    let login_action_label = if is_login_open {
        "Скрыть вход"
    } else {
        "Вход"
    };
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

                        if is_authenticated {
                            button {
                                class: "posts-page__auth-link",
                                r#type: "button",
                                onclick: move |_| {
                                    let mut next_auth = auth();
                                    next_auth.clear();
                                    auth.set(next_auth);
                                    show_login.set(false);
                                    show_registration.set(false);
                                    show_new_post.set(false);
                                    editing_post.set(None);
                                },
                                "Выйти"
                            }
                        } else {
                            button {
                                class: "posts-page__auth-link",
                                r#type: "button",
                                onclick: move |_| {
                                    let next_state = !show_login();
                                    show_login.set(next_state);
                                    if next_state {
                                        show_registration.set(false);
                                    }
                                },
                                "{login_action_label}"
                            }

                            button {
                                class: "posts-page__auth-link",
                                r#type: "button",
                                onclick: move |_| {
                                    let next_state = !show_registration();
                                    show_registration.set(next_state);
                                    if next_state {
                                        show_login.set(false);
                                    }
                                },
                                "{registration_action_label}"
                            }
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

                if is_login_open {
                    LoginModal {
                        on_close: move |_| {
                            show_login.set(false);
                        },
                        on_success: move |_| {
                            show_login.set(false);
                        }
                    }
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

                if is_new_post_open || is_edit_post_open {
                    if let Some(token) = token.clone() {
                        PostModal {
                            key: "{modal_key}",
                            token,
                            post: editing_post_value.clone(),
                            on_close: move |_| {
                                show_new_post.set(false);
                                editing_post.set(None);
                            },
                            on_success: move |_| {
                                show_new_post.set(false);
                                editing_post.set(None);
                                posts_reload_key.set(posts_reload_key() + 1);
                            }
                        }
                    }
                }

                if !is_new_post_open && !is_edit_post_open {
                    div {
                        class: "posts-page__actions",

                        button {
                            class: "posts-page__primary-action",
                            r#type: "button",
                            disabled: !is_authenticated,
                            title: if is_authenticated {
                                ""
                            } else {
                                "Кнопка доступна только после авторизации."
                            },
                            onclick: move |_| {
                                if is_authenticated {
                                    editing_post.set(None);
                                    show_new_post.set(true);
                                }
                            },
                            "Новый пост"
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
                                PostCard {
                                    key: "{post.id}",
                                    post: post.clone(),
                                    on_edit: move |post: Post| {
                                        show_new_post.set(false);
                                        editing_post.set(Some(post));
                                    }
                                }
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



