use std::ops::Deref;

use dioxus::prelude::*;
use gloo::file::BlobContents;
use web_sys::Url;

use crate::{components::atoms::{Avatar, Icon, Close, header_main::HeaderCallOptions}, services::matrix::matrix::{TimelineMessageType, EventOrigin, ImageType}};

use super::{MessageReply, header_main::HeaderEvent};

#[derive(PartialEq, Props, Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub event_id: Option<String>,
    pub content: TimelineMessageType,
    pub display_name: String,
    pub avatar_uri: Option<String>,
    pub reply: Option<MessageReply>,
    pub origin: EventOrigin,
    pub time: String
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
    let header_style = r#"
        display: flex;
        justify-content: space-between;
    "#;

    let sender_style = r#"
        color: var(--text-1);
        font-weight: 500;
    "#;

    let content_style = r#"
        font-size: var(--font-size-0);
        display: flex;
        gap: 11px;
        align-items: flex-end;
        justify-content: space-between;
    "#;

    let time_style = r#"
      color: var(--text-disabled);
      text-align: right;
      font-family: Inter;
      font-size: 10px;
      font-style: italic;
      font-weight: 400;
      line-height: 16px; /* 160% */
      letter-spacing: 0.6px;
      text-transform: uppercase;
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
          max-width: 70dvw;
          width: 100%;
        "#
      };

    let content_text_style =  match cx.props.message.origin {
      EventOrigin::ME => r#"
        color: var(--text-white);
        font-family: Inter;
        font-size: 16px;
        font-style: normal;
        font-weight: 400;
        line-height: 20px; /* 125% */
      "#,
      EventOrigin::OTHER => r#"
        color: var(--text-1);
        font-family: Inter;
        font-size: 16px;
        font-style: normal;
        font-weight: 400;
        line-height: 20px; /* 125% */
      "#
    };

    let origin_message_container_style = match cx.props.message.origin {
      EventOrigin::ME => r#"
        border-radius: 16px;
        // border: 0.5px solid var(--border-normal-50);
        background: var(--background-button);
        color: var(--text-white);
        display: flex;
        width: fit-content;
        max-width: 80%;
        margin: 0 var(--size-1) 8px auto;
        overflow: auto;
        
      "#,
      EventOrigin::OTHER => r#"
      "#
    };

    cx.render(rsx! {
      div {
        style: "{origin_message_container_style}",
        class: if !cx.props.is_replying {"message-view"} else {"message-view--replying"},
        onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
        match &cx.props.message.origin {
          EventOrigin::ME => {
            rsx!(
              div {}
            )
          },
          EventOrigin::OTHER => {
            rsx!(
              Avatar {
                name: cx.props.message.display_name.clone(),
                size: 36,
                uri: cx.props.message.avatar_uri.clone()
              }
            )
          }
        }
        article {
          style: "
            padding: 10px;
            border-radius: 16px;
            background: var(--background-message);
          ",
          match cx.props.message.origin {
            EventOrigin::OTHER => 
              rsx!(
                section {
                  style: "{header_style}",
                  span {
                    style: "{sender_style}",
                    "{cx.props.message.display_name}"
                  }
                }
             ),
            _ => rsx!(
              div {}
            )
          }

          if cx.props.message.reply.is_some() {
            rsx!(
              MessageReply{
                message: cx.props.message.reply.clone().unwrap(),
                is_replying_for_me: match cx.props.message.origin { 
                  EventOrigin::ME => true, 
                  _ => false
                }
              }
            )
          }

          match &cx.props.message.content {
            TimelineMessageType::Text(t) => {
              rsx!(
                p {
                  style: "{content_style}",
                  span {
                    style: "{content_text_style}",
                    "{t}"
                  }
                  span {
                    style: "{time_style}",
                    "{cx.props.message.time}"
                  }
                }
              )
            },
            TimelineMessageType::Image(i) => {
              match i {
                ImageType::URL(url) => {
                  rsx!(img{
                    style: "{content_image_style}",
                    src: "{url}"
                  })
                }
                ImageType::Media(content) => {
                  let c = content.deref();
                  let parts = js_sys::Array::of1(&unsafe { c.into_jsvalue() });
                  let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).unwrap();
                  let url  = Url::create_object_url_with_blob(&blob).unwrap();
                  
                  rsx!(img {
                    style: "{content_image_style}",
                    src: "{url}"
                  })
                }
              }
            },
            TimelineMessageType::Html(t) => {
              rsx!(
                div {
                  dangerous_inner_html: "{t}"
                }
              )
            }
          }          
        }

        if cx.props.is_replying {
          let close_style = r#"
            cursor: pointer;
            background: transparent;
            border: 1px solid transparent;
            display: flex;
          "#;

          rsx!(
            button {
              style: "{close_style}",
              onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
              Icon {
                stroke: "var(--icon-subdued)",
                icon: Close
              }
            }
          )
        }
      }
    })
}
