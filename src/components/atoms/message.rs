use futures_util::StreamExt;
use log::info;
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

    let message__content__image = if cx.props.is_replying {
        "message__content__image--is-replying"
    } else {
        "message__content__image--not-replying"
    };

    let message__content__video = if cx.props.is_replying {
        "message__content__video--is-replying"
    } else {
        "message__content__video--not-replying"
    };

    let message__content__text = match cx.props.message.origin {
        EventOrigin::ME => "message__content--me",
        EventOrigin::OTHER => "message__content--other",
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
                    class: "message__header",
                    span {
                      class: "message__sender",
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
                      class: "{message__content__text}",
                      "{t}"
                    }
                    span {
                      class: "message__time",
                      "{cx.props.message.time}"
                    }
                  }
                )
              },
              TimelineMessageType::Image(i) => {
                match i.source.clone().unwrap() {
                  ImageType::URL(url) => {
                    rsx!(img{
                      class: "{message__content__image}",
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
                        class: "{message__content__image}",
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
                    class: "message__content__file",
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
                        class: "{message__content__video}",
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
                        class: "{message__content__video}",
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
                let message_content_html = if cx.props.is_replying {
                  "message__content__html--is-replying"
                } else {
                  ""
                };

                rsx!(
                  div {
                    class: "{message_content_html}",
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
                  class: "file message__content__thread",
                  div {
                    class: "message__content__thread-container",
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
                    class: "message__content__thread-count",
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
