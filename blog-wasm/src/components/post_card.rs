use blog_client::Post;
use dioxus::prelude::*;

use crate::storage;

#[component]
pub(crate) fn PostCard(
    post: Post,
    on_edit: EventHandler<Post>,
    on_delete: EventHandler<Post>,
    show_delete: bool,
    is_deleting: bool,
) -> Element {
    let author_id = post.author_id.to_string();
    let auth = use_context::<Signal<storage::Auth>>();
    let auth = auth.read();
    let can_edit = auth.is_authenticated() && auth.user_id.as_deref() == Some(author_id.as_str());
    let editable_post = post.clone();
    let deletable_post = post.clone();

    rsx! {
        article {
            class: "post-card",

            div {
                class: "post-card__header",

                h2 {
                    class: "post-card__title",
                    "{post.title}"
                }

                if can_edit {
                    div {
                        class: "post-card__actions",

                        button {
                            class: "post-card__edit-action",
                            r#type: "button",
                            disabled: is_deleting,
                            onclick: move |_| on_edit.call(editable_post.clone()),
                            "Редактировать"
                        }

                        if show_delete {
                            button {
                                class: "post-card__delete-action",
                                r#type: "button",
                                disabled: is_deleting,
                                onclick: move |_| on_delete.call(deletable_post.clone()),
                                if is_deleting { "Удаление..." } else { "Удалить" }
                            }
                        }
                    }
                }
            }

            p {
                class: "post-card__content",
                "{post.content}"
            }
        }
    }
}