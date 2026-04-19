use blog_client::Post;
use dioxus::prelude::*;

use crate::api::{delete_post, fetch_posts};

use super::post_card::PostCard;
use super::post_modal::{PostModal, PostModalMode};

#[component]
pub(crate) fn PostsSection(is_authenticated: bool, token: Option<String>) -> Element {
    let mut posts_reload_key = use_signal(|| 0_u64);
    let posts_resource = use_resource(move || {
        let _ = posts_reload_key();
        fetch_posts()
    });
    let mut show_new_post = use_signal(|| false);
    let mut editing_post = use_signal(|| Option::<Post>::None);
    let mut deleting_post_id = use_signal(|| Option::<String>::None);
    let mut post_action_error = use_signal(|| Option::<String>::None);

    let is_new_post_open = show_new_post();
    let editing_post_value = editing_post();
    let is_edit_post_open = editing_post_value.is_some();
    let is_post_form_open = is_new_post_open || is_edit_post_open;

    let post_modal_mode = if let Some(post) = editing_post_value.clone() {
        Some(PostModalMode::Edit(post))
    } else if is_new_post_open {
        Some(PostModalMode::Create)
    } else {
        None
    };

    let modal_key = match &post_modal_mode {
        Some(PostModalMode::Edit(post)) => format!("edit-{}", post.id),
        Some(PostModalMode::Create) => "create".to_string(),
        None => String::new(),
    };

    rsx! {
        if let Some(mode) = post_modal_mode.clone() {
            if let Some(token) = token.clone() {
                PostModal {
                    key: "{modal_key}",
                    token,
                    mode,
                    on_close: move |_| {
                        show_new_post.set(false);
                        editing_post.set(None);
                    },
                    on_success: move |_| {
                        show_new_post.set(false);
                        editing_post.set(None);
                        post_action_error.set(None);
                        posts_reload_key.set(posts_reload_key() + 1);
                    }
                }
            }
        }

        match &*post_action_error.read() {
            Some(message) => rsx! {
                p {
                    class: "posts-page__state posts-page__state_error",
                    "{message}"
                }
            },
            None => rsx! {},
        }

        if !is_post_form_open {
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
                            post_action_error.set(None);
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
                            hide_actions: is_post_form_open,
                            is_deleting: deleting_post_id() == Some(post.id.to_string()),
                            on_edit: move |post: Post| {
                                post_action_error.set(None);
                                show_new_post.set(false);
                                editing_post.set(Some(post));
                            },
                            on_delete: {
                                let delete_token = token.clone();

                                move |post: Post| {
                                    if deleting_post_id().is_some() {
                                        return;
                                    }

                                    let Some(token) = delete_token.clone() else {
                                        return;
                                    };

                                    let post_id = post.id.to_string();

                                    post_action_error.set(None);
                                    deleting_post_id.set(Some(post_id.clone()));

                                    spawn(async move {
                                        match delete_post(token, post_id).await {
                                            Ok(()) => {
                                                deleting_post_id.set(None);
                                                posts_reload_key.set(posts_reload_key() + 1);
                                            }
                                            Err(error) => {
                                                deleting_post_id.set(None);
                                                post_action_error.set(Some(error));
                                            }
                                        }
                                    });
                                }
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