use blog_client::Post;
use dioxus::prelude::*;

use crate::storage;

#[component]
pub(crate) fn PostCard(post: Post, on_edit: EventHandler<Post>) -> Element {
    let author_id = post.author_id.to_string();
    let auth = use_context::<Signal<storage::Auth>>();
    let auth = auth.read();
    let can_edit = auth.is_authenticated() && auth.user_id.as_deref() == Some(author_id.as_str());
    let editable_post = post.clone();

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
                    button {
                        class: "post-card__edit-action",
                        r#type: "button",
                        onclick: move |_| on_edit.call(editable_post.clone()),
                        "Редактировать"
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