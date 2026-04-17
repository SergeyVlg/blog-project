use dioxus::prelude::*;

use crate::api::login_user;
use crate::storage;

#[derive(Clone, PartialEq)]
enum LoginStatus {
    Idle,
    Submitting,
    Error(String),
}

#[component]
pub(crate) fn LoginModal(on_close: EventHandler<()>, on_success: EventHandler<()>) -> Element {
    let mut login_name = use_signal(String::new);
    let mut login_password = use_signal(String::new);
    let mut login_status = use_signal(|| LoginStatus::Idle);
    let mut auth = use_context::<Signal<storage::Auth>>();

    let is_submitting = matches!(&*login_status.read(), LoginStatus::Submitting);
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
                        "Вход"
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

                        let name = login_name.read().trim().to_owned();
                        let password = login_password.read().clone();

                        if name.is_empty() || password.is_empty() {
                            login_status.set(LoginStatus::Error(
                                "Заполните имя и пароль.".into(),
                            ));
                            return;
                        }

                        login_status.set(LoginStatus::Submitting);

                        let on_success = success_handler.clone();

                        spawn(async move {
                            match login_user(name, password).await {
                                Ok(payload) => {
                                    let mut next_auth = auth();
                                    next_auth.user_id = Some(payload.user.id.to_string());
                                    next_auth.token = Some(payload.token);
                                    next_auth.save();
                                    auth.set(next_auth);

                                    login_name.set(String::new());
                                    login_password.set(String::new());
                                    login_status.set(LoginStatus::Idle);
                                    on_success.call(());
                                }
                                Err(error) => {
                                    login_status.set(LoginStatus::Error(error));
                                }
                            }
                        });
                    },

                    div {
                        class: "registration-form__fields",

                        label {
                            class: "registration-form__field",

                            span { "Имя" }

                            input {
                                value: login_name(),
                                placeholder: "Введите имя",
                                autocomplete: "username",
                                disabled: is_submitting,
                                oninput: move |event| {
                                    login_name.set(event.value());
                                    login_status.set(LoginStatus::Idle);
                                }
                            }
                        }

                        label {
                            class: "registration-form__field",

                            span { "Пароль" }

                            input {
                                value: login_password(),
                                placeholder: "Введите пароль",
                                autocomplete: "current-password",
                                disabled: is_submitting,
                                r#type: "password",
                                oninput: move |event| {
                                    login_password.set(event.value());
                                    login_status.set(LoginStatus::Idle);
                                }
                            }
                        }
                    }

                    button {
                        class: "registration-form__submit",
                        disabled: is_submitting,
                        r#type: "submit",
                        if is_submitting { "Вход..." } else { "Войти" }
                    }

                    match &*login_status.read() {
                        LoginStatus::Idle | LoginStatus::Submitting => rsx! {},
                        LoginStatus::Error(message) => rsx! {
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

