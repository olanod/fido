use futures_util::StreamExt;
use log::info;
use regex::Regex;
use ruma::MilliSecondsSinceUnixEpoch;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use dioxus::prelude::*;
use gloo::file::BlobContents;
use web_sys::Url;

use crate::{
    components::atoms::{
        hover_menu::{MenuEvent, MenuOption},
        Avatar, File, HoverMenu,
    },
    hooks::use_client::use_client,
    services::matrix::matrix::{EventOrigin, ImageType, TimelineMessageType, TimelineRelation},
};

use super::MessageReply;

#[derive(PartialEq, Debug, Clone)]
pub struct Sender {
    pub display_name: String,
    pub avatar_uri: Option<String>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ThreadPreview {
    pub meta_senders: Vec<Sender>,
    pub count: i8,
}

#[derive(PartialEq, Props, Debug, Clone)]
pub struct Message {
    pub id: i64,
    pub event_id: Option<String>,
    pub content: TimelineMessageType,
    pub display_name: String,
    pub avatar_uri: Option<String>,
    pub reply: Option<MessageReply>,
    pub origin: EventOrigin,
    pub time: String,
    pub thread: Option<ThreadPreview>,
}

#[derive(Props)]
pub struct MessageViewProps<'a> {
    pub message: Message,
    pub is_replying: bool,
    on_event: EventHandler<'a, MenuEvent>,
}

pub type Messages = Vec<TimelineRelation>;

pub fn MessageView<'a>(cx: Scope<'a, MessageViewProps<'a>>) -> Element<'a> {
    let hover_menu_options = use_ref::<Vec<MenuOption>>(cx, || match cx.props.message.thread {
        Some(_) => vec![MenuOption::ShowThread, MenuOption::Reply],
        None => vec![MenuOption::CreateThread, MenuOption::Reply],
    });

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
          max-width: 70dvw;
          width: 100%;
          max-height: calc(60vh - 30px);
          object-fit: contain;
        "#
    };

    let content_video_style = if cx.props.is_replying {
        r#"
          border-radius: var(--size-1);
          margin-top: var(--size-1);
          width: 28px;
        "#
    } else {
        r#"
          border-radius: var(--size-1);
          margin-top: var(--size-1);
          max-width: 70dvw;
          width: 100%;
          height: calc(60vh - 30px);
        "#
    };

    let content_text_style = match cx.props.message.origin {
        EventOrigin::ME => {
            r#"
              color: var(--text-white);
              font-family: Inter;
              font-size: 16px;
              font-style: normal;
              font-weight: 400;
              line-height: 20px; /* 125% */
              white-space: pre-line;
            "#
        }
        EventOrigin::OTHER => {
            r#"
              color: var(--text-1);
              font-family: Inter;
              font-size: 16px;
              font-style: normal;
              font-weight: 400;
              line-height: 20px; /* 125% */
              white-space: pre-line;
            "#
        }
    };

    let message_container = match cx.props.message.origin {
        EventOrigin::ME => "message-container",
        EventOrigin::OTHER => "",
    };

    let dropdown_left = match cx.props.message.origin {
        EventOrigin::ME => "",
        EventOrigin::OTHER => "dropdown--left",
    };

    let message_class = if !cx.props.is_replying {
        "message-view"
    } else {
        "message-view--replying"
    };

    cx.render(rsx! {
      div {
        class: "dropdown {dropdown_left}",
        div {
          class: "{message_class} {message_container}",
          // Header content (Avatar)
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
            class: "message-wrapper",
            // Name sender content
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
                    class: "message--text",
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
                match i.source.clone().unwrap() {
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

                    rsx!(
                      img {
                        style: "{content_image_style}",
                        src: "{url}"
                      }
                      a {
                        href: "{url}",
                      }
                    )
                  }
                }
              },
              TimelineMessageType::File(file) => {
                rsx!(
                  div {
                    style: "margin-top: var(--size-1);",
                    File {
                      body: file.clone()
                    }
                  }
                )
              }
              TimelineMessageType::Video(video) => {
                if !cx.props.is_replying {
                  match video.source.as_ref().unwrap() {
                    ImageType::URL(url) => {
                      rsx!(video{
                        style: "{content_video_style}",
                        src: "{url}",
                        controls: true,
                        autoplay: false
                      })
                    }
                    ImageType::Media(content) => {
                      let c = content.deref();
                      let parts = js_sys::Array::of1(&unsafe { c.into_jsvalue() });
                      let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).unwrap();
                      let url  = Url::create_object_url_with_blob(&blob).unwrap();

                      rsx!(video {
                        style: "{content_video_style}",
                        src: "{url}",
                        controls: true,
                        autoplay: false
                      })
                    }
                  }
                } else {
                  rsx!(
                    File {
                      body: video.clone()
                    }
                  )
                }
              }
              TimelineMessageType::Html(t) => {
                let html_style = if cx.props.is_replying {
                  r#"
                    overflow: hidden;
                    display: -webkit-box;
                    -webkit-line-clamp: 3;
                    -webkit-box-orient: vertical;
                  "# 
                } else {
                  ""
                };
                rsx!(
                  div {
                    style: "{html_style}",
                    dangerous_inner_html: "{t}"
                  }
                )
              }
            }
            // Thread replies
            if let Some(thread) = &cx.props.message.thread {
              // hover_menu_options.set(vec![MenuOption::ShowThread, MenuOption::Reply]);
              rsx!(
                div {
                  class: "file",
                  style: "
                    gap: 8px;
                    margin-top: var(--size-1);
                  ",
                  div {
                    style: "
                      display: flex;
                      gap: 4px;
                    ",
                    thread.meta_senders.iter().map(|t| {
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
                    style: "
                      color: var(--text-subdued);
                      font-family: Inter;
                      font-size: 14px;
                      font-style: normal;
                      font-weight: 400;
                      line-height: 20px;
                    ",
                    "{thread.count} respuestas"
                  }
                }
              )
            }
          }
        }

        if !cx.props.is_replying {
          rsx!(
            HoverMenu {
              options: hover_menu_options.read().deref().to_vec(),
              on_click: move |event: MenuEvent| {
                cx.props.on_event.call(event);
              }
            }
          )
        }
      }
    })
}
