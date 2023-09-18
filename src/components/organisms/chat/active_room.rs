use std::{collections::HashMap, ops::Deref};

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::{
        atoms::{
            header_main::{HeaderCallOptions, HeaderEvent},
            Header,
        },
        molecules::{
            input_message::{FormMessageEvent, ReplyingTo},
            rooms::CurrentRoom,
            InputMessage, List,
        },
    },
    hooks::{
        use_room::use_room, use_send_attach::use_send_attach, use_send_message::use_send_message,
    },
    pages::{
        chat::chat::{ListHeight, MessageItem},
        route::Route,
    },
    utils::i18n_get_key_value::i18n_get_key_value,
};

pub fn ActiveRoom(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    let i18n_map = HashMap::from([
        ("join-title", translate!(i18, "chat.helpers.join.title")),
        (
            "join-description",
            translate!(i18, "chat.helpers.join.description"),
        ),
        (
            "join-subtitle",
            translate!(i18, "chat.helpers.join.subtitle"),
        ),
        (
            "inputs-plain-message",
            translate!(i18, "chat.inputs.plain-message"),
        ),
        (
            "message-list-see-more",
            translate!(i18, "chat.message_list.see_more"),
        ),
    ]);

    let nav = use_navigator(cx);
    let room = use_room(cx);
    let send_message = use_send_message(cx);
    let send_attach = use_send_attach(cx);

    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();
    let height = use_shared_state::<ListHeight>(cx).unwrap();

    let input_placeholder =
        use_state::<String>(cx, || i18n_get_key_value(&i18n_map, "inputs-plain-message"));

    let on_push_message = move |evt: FormMessageEvent| {
        let mut reply_to = None;

        if let Some(r) = replying_to.read().deref() {
            reply_to = Some(r.event_id.clone());
        }

        send_message.send(MessageItem {
            room_id: room.get().id.clone(),
            msg: evt.value,
            reply_to,
        });
    };

    let header_event = move |evt: HeaderEvent| {
        to_owned![room];

        match evt.value {
            HeaderCallOptions::CLOSE => {
                nav.push(Route::ChatList {});
                room.set(CurrentRoom {
                    id: String::new(),
                    name: String::new(),
                    avatar_uri: None,
                });
            }
            _ => {}
        }
    };

    let input_message_event = move |evt: HeaderEvent| {
        to_owned![replying_to, height];

        match evt.value {
            HeaderCallOptions::CLOSE => {
                *replying_to.write() = None;
                height.write().height = "height: calc(100vh - 72px - 82px );".to_string();
            }
            _ => {}
        }
    };

    let on_handle_attach = move |event: Vec<u8>| {
        send_attach.send(event);
    };

    cx.render(rsx! {
        Header {
            text: "{room.get().name.clone()}",
            avatar_uri: None,
            on_event: header_event
        }
        List {},
        InputMessage {
            message_type: "text",
            placeholder: input_placeholder.get().as_str(),
            on_submit: on_push_message,
            on_event: input_message_event,
            on_attach: on_handle_attach
        }

    })
}
