use std::ops::Deref;

use dioxus::prelude::*;

use crate::{
    components::atoms::{Avatar, File},
    services::matrix::matrix::{ImageType, TimelineMessageType},
};

#[derive(PartialEq, Props, Debug, Clone)]
pub struct MessageReply {
    pub content: TimelineMessageType,
    pub display_name: String,
    #[props(!optional)]
    pub avatar_uri: Option<String>,
}

pub struct Sender {
    pub display_name: String,
    pub avatar_uri: String,
}

#[derive(PartialEq, Props)]
pub struct MessageReplyProps {
    pub message: MessageReply,
    pub is_replying_for_me: bool,
}

pub fn MessageReply(cx: Scope<MessageReplyProps>) -> Element {
    let message_reply_me = if cx.props.is_replying_for_me {
        "message-reply--is-replying-me"
    } else {
        "message-reply--not-replying-me"
    };

    let message_wrapper_replying_me = if cx.props.is_replying_for_me {
        "messase-reply__wrapper--is-replying-me"
    } else {
        "messase-reply__wrapper--not-replying-me"
    };

    cx.render(rsx! {
      div {
        class: "message-view--reply message_wrapper_replying_me",
        Avatar {
          name: cx.props.message.display_name.clone(),
          size: 24,
          uri: cx.props.message.avatar_uri.clone()
        }
        article {
          class: "message-reply",
          section {
            class: "message__header",
            span {
              class: "{message_reply_me}",
              "{cx.props.message.display_name}"
            }
          }
          match &cx.props.message.content {
            TimelineMessageType::Text(t) => {
              rsx!(
                p {
                  class: "message-reply__content--text",
                  "{t}"
                }
              )
            },
            TimelineMessageType::Image(i) => {
              match i.source.as_ref().unwrap() {
                ImageType::URL(url) => {
                  rsx!(img{
                    class: "message-reply__content--media",
                    src: "{url}"
                  })
                }
                ImageType::Media(content) => {
                  let c: &[u8] = content.as_ref();

                  let blob = gloo::file::Blob::new(c);
                  let object_url = gloo::file::ObjectUrl::from(blob);

                  rsx!(img{
                    class: "message-reply__content--media",
                    src: "{object_url.deref()}"
                  })
                }
              }
              // rsx!(div{})
            }
            TimelineMessageType::File(file) => {
              rsx!(
                div {
                  class: "message-reply__content--file",
                  File {
                    body: file.clone(),
                  }
                }
              )
            }
            TimelineMessageType::Video(i) => {
              rsx!(
                div {
                  class: "message-reply__content--video",
                  File {
                    body: i.clone(),
                  }
                }
              )
            }
            TimelineMessageType::Html(t) => {
              rsx!(
                div {
                  class: "message-reply__content--html",
                  dangerous_inner_html: "{t}"
                }
              )
            }
          }
        }
      }
    })
}
