use dioxus::prelude::*;

use crate::{
    components::atoms::{Avatar, ContentMessage},
    services::matrix::matrix::TimelineMessageType,
};

use super::content::content::Content;

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

#[derive(PartialEq, Props, Clone)]
pub struct MessageReplyProps {
    pub message: MessageReply,
    pub is_replying_for_me: bool,
}

pub fn MessageReply(props: MessageReplyProps) -> Element {
    let message_reply_me = if props.is_replying_for_me {
        "message-reply--is-replying-me"
    } else {
        "message-reply--not-replying-me"
    };

    let message_wrapper_replying_me = if props.is_replying_for_me {
        "messase-reply__wrapper--is-replying-me"
    } else {
        "messase-reply__wrapper--not-replying-me"
    };

    rsx! {
        div { class: "message-view--reply {message_wrapper_replying_me}",
            Avatar {
                name: props.message.display_name.clone(),
                size: 24,
                uri: props.message.avatar_uri.clone()
            }
            article { class: "message-reply",
                section { class: "message__header",
                    span { class: "{message_reply_me}", "{props.message.display_name}" }
                }
                ContentMessage {
                    message: Content {
                        content: props.message.content.clone(),
                        is_reply: true,
                        thread: None,
                    }
                }
            }
        }
    }
}
