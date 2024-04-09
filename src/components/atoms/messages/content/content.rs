use dioxus::prelude::*;

use crate::{
    components::atoms::{
        message::ThreadPreview, File, HtmlMessage, ImageMessage, TextMessage, ThreadMessage,
        VideoMessage,
    },
    services::matrix::matrix::TimelineMessageType,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Content {
    pub content: TimelineMessageType,
    pub is_reply: bool,
    pub thread: Option<ThreadPreview>,
}

#[derive(PartialEq, Props, Clone)]
pub struct ContentMessageProps {
    message: Content,
}

pub fn ContentMessage(props: ContentMessageProps) -> Element {
    rsx!(
        div {
            class: "message__content",
            match props.message.content {
                TimelineMessageType::Text(t) => {
                  rsx!(
                    TextMessage {
                      body: t,
                      is_reply: props.message.is_reply
                    }
                  )
                },
                TimelineMessageType::Image(i) => {
                  rsx!(
                    ImageMessage {
                      body: i,
                      is_reply: props.message.is_reply
                    }
                  )
                },
                TimelineMessageType::File(file) => {
                  rsx!(
                    div {
                      class: "message__content__file",
                      File {
                        body: file,
                        is_reply: props.message.is_reply
                      }
                    }
                  )
                }
                TimelineMessageType::Video(video) => {
                    rsx!(
                      VideoMessage {
                        body: video,
                        is_reply: props.message.is_reply
                      }
                    )
                }
                TimelineMessageType::Html(t) => {
                  rsx!(
                    HtmlMessage {
                      body: t,
                      is_reply: props.message.is_reply
                    }
                  )
                }
            }

            // Thread replies
            if let Some(thread) = &props.message.thread {
              // hover_menu_options.set(vec![MenuOption::ShowThread, MenuOption::Reply]);
                ThreadMessage {
                  body: thread.clone()
                }
            }
        }
    )
}
