use dioxus::prelude::*;
use blog_client::Post;

use crate::api::{create_post, update_post};

#[derive(Clone, PartialEq)]
enum PostModalStatus {
    Idle,
    Submitting,
    Error(String),
}

#[component]
pub(crate) fn PostModal(
    token: String,
    post: Option<Post>,
    on_close: EventHandler<()>,
    on_success: EventHandler<()>,
) -> Element {
    let is_editing = post.is_some();
    let post_id = post.as_ref().map(|post| post.id.to_string());
    let initial_title = post.as_ref().map(|post| post.title.clone()).unwrap_or_default();
    let initial_content = post.as_ref().map(|post| post.content.clone()).unwrap_or_default();
    let mut post_title = use_signal(move || initial_title.clone());
    let mut post_content = use_signal(move || initial_content.clone());
    let mut post_status = use_signal(|| PostModalStatus::Idle);

    let is_submitting = matches!(&*post_status.read(), PostModalStatus::Submitting);
    let close_from_backdrop = on_close.clone();
    let close_from_button = on_close.clone();
    let cancel_button = on_close.clone();
    let success_handler = on_success.clone();

    rsx! {
        div {
            class: "modal-backdrop",
            onclick: move |_| close_from_backdrop.call(()),

            div {
                class: "modal-card",
                onclick: move |event| event.stop_propagation(),

                div {
                    class: "modal-card__header",

                    h2 {
                        class: "modal-card__title",
                        if is_editing { "Редактировать пост" } else { "Новый пост" }
                    }

                    button {
                        class: "modal-card__close",
                        r#type: "button",
                        onclick: move |_| close_from_button.call(()),
                        "×"
                    }
                }

                form {
                    class: "registration-form",
                    onsubmit: move |event| {
                        event.prevent_default();

                        let title = post_title.read().trim().to_owned();
                        let content = post_content.read().trim().to_owned();

                        if title.is_empty() || content.is_empty() {
                            post_status.set(PostModalStatus::Error(
                                "Заполните заголовок и содержание поста.".into(),
                            ));
                            return;
                        }

                        post_status.set(PostModalStatus::Submitting);

                        let on_success = success_handler.clone();
                        let token = token.clone();
                        let post_id = post_id.clone();

                        spawn(async move {
                            let result = if let Some(post_id) = post_id {
                                update_post(token, post_id, title, content).await
                            } else {
                                create_post(token, title, content).await
                            };

                            match result {
                                Ok(_) => {
                                    post_status.set(PostModalStatus::Idle);
                                    on_success.call(());
                                }
                                Err(error) => {
                                    post_status.set(PostModalStatus::Error(error));
                                }
                            }
                        });
                    },

                    div {
                        class: "registration-form__fields",

                        label {
                            class: "registration-form__field",

                            span { "Заголовок" }

                            input {
                                value: post_title(),
                                placeholder: "Введите заголовок",
                                disabled: is_submitting,
                                oninput: move |event| {
                                    post_title.set(event.value());
                                    post_status.set(PostModalStatus::Idle);
                                }
                            }
                        }

                        label {
                            class: "registration-form__field",

                            span { "Содержание" }

                            textarea {
                                value: post_content(),
                                placeholder: "Введите текст поста",
                                rows: "8",
                                disabled: is_submitting,
                                oninput: move |event| {
                                    post_content.set(event.value());
                                    post_status.set(PostModalStatus::Idle);
                                }
                            }
                        }
                    }

                    div {
                        class: "modal-card__actions",

                        button {
                            class: "registration-form__submit",
                            disabled: is_submitting,
                            r#type: "submit",
                            if is_submitting { "Сохранение..." } else { "Сохранить" }
                        }

                        button {
                            class: "modal-card__secondary-action",
                            disabled: is_submitting,
                            r#type: "button",
                            onclick: move |_| cancel_button.call(()),
                            "Отмена"
                        }
                    }

                    match &*post_status.read() {
                        PostModalStatus::Idle | PostModalStatus::Submitting => rsx! {},
                        PostModalStatus::Error(message) => rsx! {
                            p {
                                class: "registration-form__status registration-form__status_error",
                                "{message}"
                            }
                        },
                    }
                }
            }
        }
    }
}

