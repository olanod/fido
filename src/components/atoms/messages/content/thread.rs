use dioxus::prelude::*;

use crate::components::atoms::{message::ThreadPreview, Avatar};

#[derive(PartialEq, Props, Clone)]
pub struct ThreadProps {
    body: ThreadPreview,
}

pub fn ThreadMessage(props: ThreadProps) -> Element {
    rsx!(
        div { class: "file message__content__thread",
            div { class: "message__content__thread-container",
                {
                    props.body.meta_senders.iter().map(|t| rsx!(
                        Avatar {
                            name: t.display_name.clone(),
                            size: 16,
                            uri: t.avatar_uri.clone()
                        }
                    ))
                }
            }
            span { class: "message__content__thread-count", "{props.body.count} respuestas" }
        }
    )
}
