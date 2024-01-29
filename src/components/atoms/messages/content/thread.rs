use dioxus::prelude::*;

use crate::components::atoms::{message::ThreadPreview, Avatar};

#[derive(PartialEq, Props)]
pub struct ThreadProps<'a> {
    body: &'a ThreadPreview,
}

pub fn ThreadMessage<'a>(cx: Scope<'a, ThreadProps<'a>>) -> Element<'a> {
    render!(rsx!(
      div {
        class: "file message__content__thread",
        div {
          class: "message__content__thread-container",
          cx.props.body.meta_senders.iter().map(|t| {
            rsx!(
                Avatar {
                  name: t.display_name.clone(),
                  size: 16,
                  uri: t.avatar_uri.clone()
                }
              )
            })
        }
        span {
          class: "message__content__thread-count",
          "{cx.props.body.count} respuestas"
        }
      }
    ))
}
