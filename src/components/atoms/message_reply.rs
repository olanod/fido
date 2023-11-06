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
    let message_style = r#"
        width: calc(100% - 30px);
    "#;

    let header_style = r#"
        display: flex;
        justify-content: space-between;
    "#;

    let sender_style = if cx.props.is_replying_for_me {
        r#"
        color: var(--text-white);
        font-family: Inter;
        font-size: 12px;
        font-weight: 500;
        line-height: 12px; 
      "#
    } else {
        r#"
        color: var(--text-1);
        font-family: Inter;
        font-size: 12px;
        font-weight: 500;
        line-height: 12px;
      "#
    };

    let content_style = r#"
        margin-top: var(--size-0);
        font-size: var(--font-size-0);
    "#;

    let content_image_style = r#"
      border-radius: var(--size-1);
      margin-top: var(--size-1);
      width: 28px;
    "#;

    let message_wrapper_style = if cx.props.is_replying_for_me {
        r#"
        background: transparent;
        color: var(--text-white);
        margin: var(--size-0) 0;
        padding: 0 var(--size-0);
        border-left: 2px solid var(--primary-100);
      "#
    } else {
        r#"
        margin: var(--size-0) 0;
        padding: 0 var(--size-0);
        border-left: 2px solid var(--primary-100);
      "#
    };

    cx.render(rsx! {
      div {
        class: "message-view--reply",
        style: "{message_wrapper_style}",
        Avatar {
          name: cx.props.message.display_name.clone(),
          size: 24,
          uri: cx.props.message.avatar_uri.clone()
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
              match i.source.as_ref().unwrap() {
                ImageType::URL(url) => {
                  rsx!(img{
                    style: "{content_image_style}",
                    src: "{url}"
                  })
                }
                ImageType::Media(content) => {
                  let c: &[u8] = content.as_ref();

                  let blob = gloo::file::Blob::new(c);
                  let object_url = gloo::file::ObjectUrl::from(blob);

                  rsx!(img{
                    style: "{content_image_style}",
                    src: "{object_url.deref()}"
                  })
                }
              }
              // rsx!(div{})
            }
            TimelineMessageType::File(file) => {
              rsx!(
                div {
                  style: "margin-top: var(--size-0)",
                  File {
                    body: file.clone(),
                  }
                }
              )
            }
            TimelineMessageType::Video(i) => {
              rsx!(
                div {
                  style: "margin-top: var(--size-0)",
                  File {
                    body: i.clone(),
                  }
                }
              )
            }
            TimelineMessageType::Html(t) => {
              rsx!(
                div {
                  style: "
                    overflow: hidden;
                    display: -webkit-box;
                    -webkit-line-clamp: 3;
                    -webkit-box-orient: vertical;
                  ",
                  dangerous_inner_html: "{t}"
                }
              )
            }
            TimelineMessageType::Payment(p) => {
              rsx!(
                div {
                  style: "
                    
                  ",
                  p {
                    style: "

                    ",
                    "Payment to Vitalik"
                  }
                  div {
                    style: "
                      display: flex;
                      justify-content: space-between;
                      align-items: center;
                      margin-top: 2px;
                    ",
                    span {
                      style: "
                        color: #E3E3E3;
                        font-family: Inter;
                        font-size: 32px;
                        font-style: normal;
                        font-weight: 500;
                        line-height: 40px;
                        letter-spacing: -0.32px;
                      ",
                      "{p.value} {p.asset}"
                    }
                    span {
                      style: "
                        color: var(--icon-normal);
                        font-family: Inter;
                        font-size: 16px;
                        font-style: normal;
                        font-weight: 500;
                        line-height: 24px;
                      ",
                      "$20"
                    }
                  }
                }
              )
            }
          }
        }
      }
    })
}
