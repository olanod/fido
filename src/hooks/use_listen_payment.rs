use std::ops::Deref;

use chrono::{DateTime, Local};
use dioxus::prelude::*;
use futures_util::StreamExt;
use log::info;
use matrix_sdk::{
    config::SyncSettings, room::Room, ruma::events::room::message::OriginalSyncRoomMessageEvent,
};
use ruma::events::{room::message::Relation, OriginalSyncMessageLikeEvent};
use std::{
    ptr::eq,
    time::{Duration, UNIX_EPOCH},
};

use crate::services::matrix::matrix::TimelineMessage;
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
        format_head_thread, format_original_any_room_message_event, format_relation_from_event,
        room_member, EventOrigin, OriginalSyncPaymentEvent, PaymentEventContent, SyncPaymentEvent,
        TimelineMessageType, TimelineRelation, TimelineThread,
    },
};

use super::{use_client::use_client, use_notification::use_notification, use_room::use_room};

#[allow(clippy::needless_return)]
pub fn use_listen_payment(cx: &ScopeState) -> &UseListenPaymentState {
    let client = use_client(cx).get();
    let notification = use_notification(cx);
    let room = use_room(cx);
    let messages = use_messages(cx);

    let handler_added = use_ref(cx, || false);
    let timeline_thread =
        use_shared_state::<Option<TimelineThread>>(cx).expect("Unable to use TimelineThread");

    let task_sender = use_coroutine(cx, |mut rx: UnboundedReceiver<MessageEvent>| {
        to_owned![client, messages, notification, room, timeline_thread];

        async move {
            while let Some(message_event) = rx.next().await {
                if let Some(message) = message_event.mgs {
                    let mut msgs = messages.get().clone();
                    let mut plain_message = "";

                    let is_in_current_room =
                        message_event.room.room_id().as_str().eq(&room.get().id);

                    let last_message_id = messages.get().len() as i64;

                    match &message {
                        TimelineRelation::Thread(x) => {
                            // Position of an existing thread timeline

                            let position = msgs.iter().position(|m| {
                                if let TimelineRelation::CustomThread(y) = m {
                                    y.event_id.eq(&x.event_id)
                                } else {
                                    false
                                }
                            });

                            if let Some(p) = position {
                                if let TimelineRelation::CustomThread(ref mut z) = msgs[p] {
                                    z.thread.push(x.thread[0].clone());
                                    z.thread.rotate_right(1)
                                };
                            } else {
                                let n = TimelineRelation::CustomThread(TimelineThread {
                                    event_id: x.event_id.clone(),
                                    thread: x.thread.clone(),
                                    latest_event: match x.thread[x.thread.len() - 1]
                                        .clone()
                                        .event_id
                                    {
                                        Some(id) => id,
                                        None => {
                                            notification
                                                .handle_error("Error inesperado: (Id de hilo)");
                                            return;
                                        }
                                    },
                                    count: x.thread.len(),
                                });

                                if is_in_current_room {
                                    msgs.push(n);
                                    msgs.rotate_right(1);
                                }
                            }

                            plain_message = "Nuevo mensaje en el hilo";
                        }
                        TimelineRelation::None(x) => {
                            // Position of a head thread timeline
                            let position = msgs.iter().position(|m| {
                                if let TimelineRelation::CustomThread(y) = m {
                                    match &x.event_id {
                                        Some(id) => y.event_id.eq(id),
                                        None => {
                                            notification
                                                .handle_error("Error inesperado: (Id de evento)");
                                            false
                                        }
                                    }
                                } else {
                                    false
                                }
                            });

                            if let Some(p) = position {
                                if let TimelineRelation::CustomThread(ref mut z) = msgs[p] {};
                            } else {
                                if is_in_current_room {
                                    msgs.push(message.clone());
                                    msgs.rotate_right(1);
                                    info!("after push");
                                }

                                plain_message = match &x.body {
                                    TimelineMessageType::Image(_) => "Imagen",
                                    TimelineMessageType::Text(t) => t,
                                    TimelineMessageType::File(t) => "Archivo adjunto",
                                    TimelineMessageType::Video(t) => "Video",
                                    TimelineMessageType::Html(t) => "Bloque de texto",
                                    TimelineMessageType::Payment(t) => "Nuevo pago",
                                };
                            }
                        }
                        TimelineRelation::Reply(x) => {
                            if is_in_current_room {
                                msgs.push(message.clone());
                                msgs.rotate_right(1);
                            }

                            plain_message = match &x.event.body {
                                TimelineMessageType::Image(_) => "Imagen",
                                TimelineMessageType::Text(t) => t,
                                TimelineMessageType::File(t) => "Archivo adjunto",
                                TimelineMessageType::Video(t) => "Video",
                                TimelineMessageType::Html(t) => "Bloque de texto",
                                TimelineMessageType::Payment(t) => "Nuevo pago",
                            };
                        }
                        TimelineRelation::CustomThread(x) => {
                            if is_in_current_room {
                                msgs.push(message);
                                msgs.rotate_right(1);
                            }

                            plain_message = "Nuevo mensaje en el hilo";
                        }
                    };
                    info!("before write");
                    messages.set(msgs.clone());

                    info!(
                        "all messages listen message 167: {:#?}",
                        messages.get().deref()
                    );
                    let mm = timeline_thread.read().clone();

                    if let Some(thread) = mm {
                        let ms = messages.get().clone();
                        let message = ms.iter().find(|m| {
                            if let TimelineRelation::CustomThread(t) = m {
                                if t.event_id.eq(&thread.event_id) {
                                    let mut xthread = thread.clone();

                                    info!("timeline when use messages: {t:#?}");

                                    // xthread.thread.append(&mut t.thread.clone());

                                    *timeline_thread.write() = Some(TimelineThread {
                                        event_id: t.event_id.clone(),
                                        thread: t.thread.clone(),
                                        count: t.count.clone(),
                                        latest_event: t.latest_event.clone(),
                                    });
                                }

                                true
                            } else if let TimelineRelation::None(t) = m {
                                if let Some(event_id) = &t.event_id {
                                    if event_id.eq(&thread.event_id) {
                                        let mut xthread = thread.clone();

                                        info!("timeline when use messages: {t:#?}");

                                        xthread.thread.push(t.clone());

                                        *timeline_thread.write() = Some(TimelineThread {
                                            event_id: event_id.clone(),
                                            thread: xthread.thread.clone(),
                                            count: xthread.count.clone(),
                                            latest_event: xthread.latest_event.clone(),
                                        });

                                        true
                                    } else {
                                        false
                                    }
                                } else {
                                    false
                                }
                            } else {
                                false
                            }
                        });
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
            client.sync_once(SyncSettings::default()).await;

            if !*handler_added.read() {
                client.add_event_handler(
                    move |ev: OriginalSyncPaymentEvent, room: Room, client: matrix_sdk::Client| {
                        let task_sender = task_sender.clone();

                        info!("listen messages payment {:#?}", ev);

                        let user = client.user_id();

                        let me = match user {
                            Some(u) => u.to_string(),
                            None => {
                                panic!("User not found");
                            }
                        };

                        async move {
                            let member = room_member(ev.sender, &room).await;
                            let time = ev.origin_server_ts;

                            let event = ev.event_id;

                            if let Some(x) = ev.unsigned.relations {}

                            let timestamp = {
                                let d = UNIX_EPOCH + Duration::from_millis(time.0.into());

                                let datetime = DateTime::<Local>::from(d);
                                datetime.format("%H:%M").to_string()
                            };

                            let message_result = Some(TimelineRelation::None(TimelineMessage {
                                event_id: Some(String::from(event.as_str())),
                                sender: member.clone(),
                                body: TimelineMessageType::Payment(ev.content),
                                origin: if member.id.eq(&me) {
                                    EventOrigin::ME
                                } else {
                                    EventOrigin::OTHER
                                },
                                time: timestamp,
                            }));

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

    cx.use_hook(move || UseListenPaymentState {})
}

#[derive(Clone)]
pub struct UseListenPaymentState {}

impl UseListenPaymentState {
    pub fn initialize(&self) {}
}
