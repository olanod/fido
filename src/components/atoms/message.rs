use dioxus::prelude::*;

use crate::{components::atoms::{Avatar, header::HeaderCallOptions}, services::matrix::matrix::TimelineMessageType};

use super::{MessageReply, header::HeaderEvent};

#[derive(PartialEq, Props, Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub event_id: Option<String>,
    pub content: TimelineMessageType,
    pub display_name: String,
    pub avatar_uri: Option<String>,
    pub reply: Option<MessageReply>,
}

#[derive(Debug, Clone)]
pub struct Sender {
    pub display_name: String,
    pub avatar_uri: String,
}

#[derive(Props)]
pub struct MessageViewProps<'a> {
    pub message: Message,
    pub is_replying: bool,
    on_event: EventHandler<'a, HeaderEvent>,
}

pub type Messages = Vec<Message>;

pub fn MessageView<'a>(cx: Scope<'a, MessageViewProps<'a>>) -> Element<'a> {
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

    let content_image_style = if cx.props.is_replying { 
        r#"
          border-radius: var(--size-1);
          margin-top: var(--size-1);
          width: 28px;
        "#
      } else {
        r#"
          border-radius: var(--size-1);
          margin-top: var(--size-1);
          max-height: 100dvh;
        "#
      };

    cx.render(rsx! {
      div {
        // class: "message-view",
        class: if !cx.props.is_replying {"message-view"} else {"message-view--replying"},
        Avatar {
          name: "{cx.props.message.display_name}",
          size: 36,
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

          if cx.props.message.reply.is_some() {
            rsx!(
              MessageReply{
                message: cx.props.message.reply.clone().unwrap()
              }
            )
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
            } 
          }

          
        }
        if cx.props.is_replying {
          let close_style = r#"
            cursor: pointer;
            background: transparent;
            border: 1px solid transparent;
          "#;

          let icon_style = r#"
            fill: var(--text-1)
          "#;

          rsx!(
            button {
              style: "{close_style}",
              onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
              svg {
                style: "{icon_style}",
                view_box: "0 0 50 50",
                height: 16,
                width: 16,
                path {
                    d: "M 9.15625 6.3125 L 6.3125 9.15625 L 22.15625 25 L 6.21875 40.96875 L 9.03125 43.78125 L 25 27.84375 L 40.9375 43.78125 L 43.78125 40.9375 L 27.84375 25 L 43.6875 9.15625 L 40.84375 6.3125 L 25 22.15625 Z"
                }
              }
            }
          )
        }
      }
    })
}
