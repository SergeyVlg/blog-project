use dioxus::prelude::*;

use crate::api::create_post;
use crate::storage;

#[derive(Clone, PartialEq)]
enum CreatePostStatus {
    Idle,
    Submitting,
    Error(String),
}

#[component]
pub(crate) fn CreatePostModal(token: String, on_close: EventHandler<()>, on_success: EventHandler<()>) -> Element {
    let mut post_title = use_signal(String::new);
    let mut post_content = use_signal(String::new);
    let mut post_status = use_signal(|| CreatePostStatus::Idle);

    let is_submitting = matches!(&*post_status.read(), CreatePostStatus::Submitting);
    let close_from_backdrop = on_close.clone();
    let close_from_button = on_close.clone();
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
                        "Новый пост"
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
                            post_status.set(CreatePostStatus::Error(
                                "Заполните заголовок и содержание поста.".into(),
                            ));
                            return;
                        }

                        post_status.set(CreatePostStatus::Submitting);

                        let on_success = success_handler.clone();
                        let token = token.clone();

                        spawn(async move {
                            match create_post(token, title, content).await {
                                Ok(_) => {
                                    post_title.set(String::new());
                                    post_content.set(String::new());
                                    post_status.set(CreatePostStatus::Idle);
                                    on_success.call(());
                                }
                                Err(error) => {
                                    post_status.set(CreatePostStatus::Error(error));
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
                                    post_status.set(CreatePostStatus::Idle);
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
                                    post_status.set(CreatePostStatus::Idle);
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
                            onclick: move |_| on_close.call(()),
                            "Закрыть"
                        }
                    }

                    match &*post_status.read() {
                        CreatePostStatus::Idle | CreatePostStatus::Submitting => rsx! {},
                        CreatePostStatus::Error(message) => rsx! {
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

