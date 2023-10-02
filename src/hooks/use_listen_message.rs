use dioxus::prelude::*;
use futures_util::StreamExt;
use log::info;
use matrix_sdk::{
    config::SyncSettings, room::Room, ruma::events::room::message::OriginalSyncRoomMessageEvent,
};

use crate::{
    components::{
        atoms::{
            message::{Message, Messages},
            MessageReply,
        },
        molecules::rooms::CurrentRoom,
        organisms::chat::utils::handle_notification,
    },
    pages::chat::chat::{MessageEvent, NotificationHandle, NotificationItem, NotificationType},
    services::matrix::matrix::{
        format_original_any_room_message_event, format_reply_from_event, room_member,
        TimelineMessageType,
    },
};

use super::{use_client::use_client, use_notification::use_notification};

#[allow(clippy::needless_return)]
pub fn use_listen_message(cx: &ScopeState) -> &UseListenMessagesState {
    let client = use_client(cx).get();
    let messages = use_shared_state::<Messages>(cx).unwrap();
    let current_room = use_shared_state::<CurrentRoom>(cx).unwrap();
    let handler_added = use_ref(cx, || false);
    let notification = use_notification(cx);

    let task_sender = use_coroutine(cx, |mut rx: UnboundedReceiver<MessageEvent>| {
        to_owned![client, messages, notification, current_room];

        async move {
            while let Some(message_event) = rx.next().await {
                if let Some(message) = message_event.mgs {
                    let mut reply = None;

                    if let Some(r) = message.reply {
                        reply = Some(MessageReply {
                            content: r.body,
                            display_name: r.sender.name,
                            avatar_uri: r.sender.avatar_uri,
                        });
                    }

                    let plain_message = match &message.body {
                        TimelineMessageType::Image(_) => "Imagen",
                        TimelineMessageType::Text(t) => t,
                        TimelineMessageType::Html(t) => todo!(),
                    };

                    let room_name = if let Some(name) = message_event.room.name() {
                        name
                    } else {
                        let mut name = String::from("Unknown name room");
                        let me = client.whoami().await.unwrap();
                        let users = message_event.room.members().await;

                        if let Ok(members) = users {
                            let member = members
                                .into_iter()
                                .find(|member| !member.user_id().eq(&me.user_id));

                            if let Some(m) = member {
                                let n = m.name();

                                name = String::from(n);
                            }
                        }

                        name
                    };

                    handle_notification(
                        NotificationItem {
                            title: String::from(room_name),
                            body: String::from(plain_message),
                            show: true,
                            handle: NotificationHandle {
                                value: NotificationType::Click,
                            },
                        },
                        notification.to_owned(),
                    );

                    let is_in_current_room = message_event
                        .room
                        .room_id()
                        .as_str()
                        .eq(&current_room.read().id);

                    let last_message_id = messages.read().len() as i64;

                    if is_in_current_room {
                        messages.write().push(Message {
                            id: last_message_id,
                            event_id: message.event_id,
                            display_name: message.sender.name.clone(),
                            content: message.body.clone(),
                            avatar_uri: message.sender.avatar_uri.clone(),
                            reply: reply,
                            origin: message.origin.clone(),
                            time: message.time.clone(),
                        });

                        messages.write().rotate_right(1);
                    }
                }
            }
        }
    })
    .clone();

    // After logging is mandatory to perform a client sync,
    // since the chat needs sync to listen for new messages
    // this coroutine is necesary
    use_coroutine(cx, |_: UnboundedReceiver<String>| {
        to_owned![client, handler_added, task_sender];

        async move {
            // let user = client.whoami().await;

            client.sync_once(SyncSettings::default()).await;

            // let user = client.user_id();

            // let me = match user {
            //     Ok(u) => u.user_id.to_string(),
            //     Err(_) => {
            //         panic!("User not found");
            //     }
            // };

            // let me = match user {
            //     Some(u) => u.to_string(),
            //     None => {
            //         panic!("User not found");
            //     }
            // };

            // let me = String::from("@edith-test-1:matrix.org");

            if !*handler_added.read() {
                client.add_event_handler(
                    move |ev: OriginalSyncRoomMessageEvent,
                          room: Room,
                          client: matrix_sdk::Client| {
                        let task_sender = task_sender.clone();

                        let user = client.user_id();

                        let me = match user {
                            Some(u) => u.to_string(),
                            None => {
                                panic!("User not found");
                            }
                        };
                        
                        async move {
                            let message_type = &ev.content.msgtype;
                            let event_id = ev.event_id;
                            let member = room_member(ev.sender, &room).await;
                            let relates = &ev.content.relates_to;
                            let time = ev.origin_server_ts;

                            let message_result = format_original_any_room_message_event(
                                &message_type,
                                event_id,
                                &member,
                                &me,
                                time,
                            )
                            .await;

                            let message_result = format_reply_from_event(
                                &message_type,
                                relates,
                                &room,
                                message_result,
                                &member,
                                &me,
                                time,
                            )
                            .await;

                            task_sender.send(MessageEvent {
                                room,
                                mgs: message_result,
                            })
                        }
                    },
                );

                handler_added.set(true);
            }

            let _ = client.sync(SyncSettings::default()).await;
        }
    });

    cx.use_hook(move || UseListenMessagesState {
        inner: current_room.clone(),
    })
}

#[derive(Clone)]
pub struct UseListenMessagesState {
    inner: UseSharedState<CurrentRoom>,
}

impl UseListenMessagesState {
    pub fn initialize(&self) {}
}
