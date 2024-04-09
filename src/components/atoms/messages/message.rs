use std::ops::Deref;

use dioxus::prelude::*;

use crate::{
    components::atoms::{
        content::content::Content,
        hover_menu::{MenuEvent, MenuOption},
        Avatar, ContentMessage, HoverMenu,
    },
    services::matrix::matrix::{EventOrigin, TimelineMessageType, TimelineRelation},
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
    pub event_id: String,
    pub content: TimelineMessageType,
    pub display_name: String,
    pub avatar_uri: Option<String>,
    pub reply: Option<MessageReply>,
    pub origin: EventOrigin,
    pub time: String,
    pub thread: Option<ThreadPreview>,
}

#[derive(PartialEq, Props, Clone)]
pub struct MessageViewProps {
    pub message: Message,
    pub is_replying: bool,
    on_event: EventHandler<MenuEvent>,
}

pub type Messages = Vec<TimelineRelation>;

pub fn MessageView(props: MessageViewProps) -> Element {
    let hover_menu_options = use_signal::<Vec<MenuOption>>(|| match props.message.thread {
        Some(_) => vec![MenuOption::ShowThread, MenuOption::Reply],
        None => vec![MenuOption::CreateThread, MenuOption::Reply],
    });

    let message_container = match props.message.origin {
        EventOrigin::ME => "message-container",
        EventOrigin::OTHER => "",
    };

    let dropdown_left = match props.message.origin {
        EventOrigin::ME => "",
        EventOrigin::OTHER => "dropdown--left",
    };

    let message_class = if !props.is_replying {
        "message-view"
    } else {
        "message-view--replying"
    };

    let content = Content {
        content: props.message.content.clone(),
        is_reply: props.is_replying,
        thread: props.message.thread.clone(),
    };

    rsx! {
        div { class: "dropdown {dropdown_left}",
            div { class: "{message_class} {message_container}",
                // Header content (Avatar)
                match &props.message.origin {
                    EventOrigin::ME => None,
                    EventOrigin::OTHER => rsx!(
                        Avatar {
                            name: props.message.display_name.clone(),
                            size: 36,
                            uri: props.message.avatar_uri.clone()
                        }
                    )
                },
                article { class: "message-wrapper",
                    // Name sender content
                    match props.message.origin {
                        EventOrigin::OTHER => rsx!(
                            section { class: "message__header",
                                span { class: "message__sender", "{props.message.display_name}"}
                            }
                        ),
                        _ => None
                    },
                    {
                        props.message.reply.as_ref().map(|reply| rsx!(
                            MessageReply{
                                message: reply.clone(),
                                is_replying_for_me: matches!(props.message.origin, EventOrigin::ME)
                            }
                        ))
                    },

                    div { class: "message__container__content",
                        ContentMessage { message: content.clone() }
                        span { class: "message__time", "{props.message.time}" }
                    }
                }
            }

            if !props.is_replying {
                HoverMenu {
                    options: hover_menu_options.read().deref().to_vec(),
                    on_click: move |event: MenuEvent| {
                        props.on_event.call(event);
                    }
                }
            }
        }
    }
}
