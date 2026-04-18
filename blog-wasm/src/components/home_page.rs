use dioxus::prelude::*;
use crate::storage;

use super::login_modal::LoginModal;
use super::posts_section::PostsSection;
use super::registration_modal::RegistrationModal;

#[component]
pub fn HomePage() -> Element {
    let mut show_login = use_signal(|| false);
    let mut show_registration = use_signal(|| false);
    let mut registration_success = use_signal(|| Option::<String>::None);
    let mut auth = use_signal(storage::Auth::new);

    use_context_provider(|| auth);

    let is_login_open = show_login();
    let is_registration_open = show_registration();
    let auth_state = auth();
    let token = auth_state.token.clone();
    let current_user_name = auth_state.user_name.clone();
    let is_authenticated = auth_state.is_authenticated();
    let posts_section_key = match (&auth_state.user_id, &auth_state.token) {
        (Some(user_id), Some(_)) => format!("user-{user_id}"),
        _ => "guest".to_string(),
    };

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
                            if let Some(user_name) = current_user_name.clone() {
                                span {
                                    class: "posts-page__auth-status",
                                    "Вы вошли как {user_name}"
                                }
                            }

                            button {
                                class: "posts-page__auth-link",
                                r#type: "button",
                                onclick: move |_| {
                                    let mut next_auth = auth();
                                    next_auth.clear();
                                    auth.set(next_auth);
                                    show_login.set(false);
                                    show_registration.set(false);
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

                PostsSection {
                    key: "{posts_section_key}",
                    is_authenticated,
                    token,
                }
            }
        }
    }
}