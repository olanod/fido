use dioxus::prelude::*;

use crate::{components::atoms::Avatar, services::matrix::matrix::TimelineMessageType};

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
}

pub fn MessageReply(cx: Scope<MessageReplyProps>) -> Element {
    let message_wrapper_style = r#"
      margin: var(--size-0) 0;
      padding: 0 var(--size-0);
      border-left: 2px solid var(--brand);
    "#;

    let message_style = r#"
        width: calc(100% - 30px);
    "#;

    let header_style = r#"
        display: flex;
        justify-content: space-between;
    "#;

    let sender_style = r#"
        color: var(--text-1);
    "#;

    let content_style = r#"
        margin-top: var(--size-0);
        font-size: var(--font-size-0);
    "#;

    let content_image_style = r#"
      border-radius: var(--size-1);
      margin-top: var(--size-1);
      width: 28px;
    "#;

    cx.render(rsx! {
      div {
        class: "message-view--reply",
        style: "{message_wrapper_style}",
        Avatar {
          name: "{cx.props.message.display_name}",
          size: 24,
          uri: cx.props.message.avatar_uri.as_ref()
        }
        article {
          style: "{message_style}",
          section {
            style: "{header_style}",
            span {
              style: "{sender_style}",
              "{cx.props.message.display_name}"
            }
          }
          match &cx.props.message.content {
            TimelineMessageType::Text(t) => {
              rsx!(
                p {
                  style: "{content_style}",
                  "{t}"
                }
              )
            },
            TimelineMessageType::Image(i) => {
              rsx!(img{
                style: "{content_image_style}",
                src: "{i}"
              })
              // rsx!(div{})
            }
          }
        }
      }
    })
}
