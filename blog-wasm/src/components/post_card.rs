use blog_client::Post;
use dioxus::prelude::*;

#[component]
pub(crate) fn PostCard(post: Post) -> Element {
    rsx! {
        article {
            class: "post-card",

            h2 {
                class: "post-card__title",
                "{post.title}"
            }

            p {
                class: "post-card__content",
                "{post.content}"
            }
        }
    }
}