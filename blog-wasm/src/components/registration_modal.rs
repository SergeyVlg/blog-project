use dioxus::prelude::*;

use crate::api::register_user;

#[derive(Clone, PartialEq)]
enum RegistrationStatus {
    Idle,
    Submitting,
    Error(String),
}

#[component]
pub(crate) fn RegistrationModal(on_close: EventHandler<()>, on_success: EventHandler<String>) -> Element {
    let mut registration_name = use_signal(String::new);
    let mut registration_email = use_signal(String::new);
    let mut registration_password = use_signal(String::new);
    let mut registration_status = use_signal(|| RegistrationStatus::Idle);

    let is_submitting = matches!(&*registration_status.read(), RegistrationStatus::Submitting);
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
                        "Регистрация"
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

                        let name = registration_name.read().trim().to_owned();
                        let email = registration_email.read().trim().to_owned();
                        let password = registration_password.read().clone();

                        if name.is_empty() || email.is_empty() || password.is_empty() {
                            registration_status.set(RegistrationStatus::Error(
                                "Заполните имя, электронную почту и пароль.".into(),
                            ));
                            return;
                        }

                        registration_status.set(RegistrationStatus::Submitting);

                        let on_success = success_handler.clone();

                        spawn(async move {
                            match register_user(name, email, password).await {
                                Ok(message) => {
                                    registration_name.set(String::new());
                                    registration_email.set(String::new());
                                    registration_password.set(String::new());
                                    registration_status.set(RegistrationStatus::Idle);
                                    on_success.call(message);
                                }
                                Err(error) => {
                                    registration_status.set(RegistrationStatus::Error(error));
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
                                value: registration_name(),
                                placeholder: "Введите имя",
                                autocomplete: "username",
                                disabled: is_submitting,
                                oninput: move |event| {
                                    registration_name.set(event.value());
                                    registration_status.set(RegistrationStatus::Idle);
                                }
                            }
                        }

                        label {
                            class: "registration-form__field",

                            span { "Электронная почта" }

                            input {
                                value: registration_email(),
                                placeholder: "name@example.com",
                                autocomplete: "email",
                                disabled: is_submitting,
                                r#type: "email",
                                oninput: move |event| {
                                    registration_email.set(event.value());
                                    registration_status.set(RegistrationStatus::Idle);
                                }
                            }
                        }

                        label {
                            class: "registration-form__field",

                            span { "Пароль" }

                            input {
                                value: registration_password(),
                                placeholder: "Введите пароль",
                                autocomplete: "new-password",
                                disabled: is_submitting,
                                r#type: "password",
                                oninput: move |event| {
                                    registration_password.set(event.value());
                                    registration_status.set(RegistrationStatus::Idle);
                                }
                            }
                        }
                    }

                    button {
                        class: "registration-form__submit",
                        disabled: is_submitting,
                        r#type: "submit",
                        if is_submitting { "Регистрация..." } else { "Зарегистрировать" }
                    }

                    match &*registration_status.read() {
                        RegistrationStatus::Idle | RegistrationStatus::Submitting => rsx! {},
                        RegistrationStatus::Error(message) => rsx! {
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

