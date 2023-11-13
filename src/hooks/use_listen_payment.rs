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

use super::{use_client::use_client, use_notification::use_notification};

#[allow(clippy::needless_return)]
pub fn use_listen_payment(cx: &ScopeState) -> &UseListenPaymentState {
    let client = use_client(cx).get();
    let messages = use_shared_state::<Messages>(cx).unwrap();
    let current_room = use_shared_state::<CurrentRoom>(cx).unwrap();
    let handler_added = use_ref(cx, || false);
    let notification = use_notification(cx);
    let timeline_thread = use_shared_state::<Option<TimelineThread>>(cx).unwrap();

    let task_sender = use_coroutine(cx, |mut rx: UnboundedReceiver<MessageEvent>| {
        to_owned![
            client,
            messages,
            notification,
            current_room,
            timeline_thread
        ];

        async move {
            while let Some(message_event) = rx.next().await {
                if let Some(message) = message_event.mgs {
                    let mut msgs = messages.read().clone();
                    let mut plain_message = "";

                    let is_in_current_room = message_event
                        .room
                        .room_id()
                        .as_str()
                        .eq(&current_room.read().id);

                    let last_message_id = messages.read().len() as i64;

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
                                    latest_event: x.thread[x.thread.len() - 1]
                                        .clone()
                                        .event_id
                                        .unwrap(),
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
                                    y.event_id.eq(x.event_id.as_ref().unwrap())
                                } else {
                                    false
                                }
                            });

                            if let Some(p) = position {
                                if let TimelineRelation::CustomThread(ref mut z) = msgs[p] {
                                    // let mm = format_head_thread(zz.event.deserialize().unwrap());

                                    // if let Some(x) = mm {
                                    //     z.latest_event = x.1;
                                    // }
                                    // z.thread.push(x.clone());
                                };
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
                    *messages.write() = msgs.clone();

                    info!(
                        "all messages listen message 167: {:#?}",
                        messages.read().deref()
                    );
                    let mm = timeline_thread.read().clone();

                    if let Some(thread) = mm {
                        let ms = messages.read().deref().clone();
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
                    info!("after write");
                    // let room_name = if let Some(name) = message_event.room.name() {
                    //     name
                    // } else {
                    //     let mut name = String::from("Unknown name room");
                    //     let me = client.whoami().await.unwrap();
                    //     let users = message_event.room.members().await;

                    //     if let Ok(members) = users {
                    //         let member = members
                    //             .into_iter()
                    //             .find(|member| !member.user_id().eq(&me.user_id));

                    //         if let Some(m) = member {
                    //             let n = m.name();

                    //             name = String::from(n);
                    //         }
                    //     }

                    //     name
                    // };

                    // handle_notification(
                    //     NotificationItem {
                    //         title: String::from(room_name),
                    //         body: String::from(plain_message),
                    //         show: true,
                    //         handle: NotificationHandle {
                    //             value: NotificationType::Click,
                    //         },
                    //     },
                    //     notification.to_owned(),
                    // );
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

                            if let Some(x) = ev.unsigned.relations {
                                // x.thread.unwrap().latest_event
                            }

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

    cx.use_hook(move || UseListenPaymentState {
        inner: current_room.clone(),
    })
}

#[derive(Clone)]
pub struct UseListenPaymentState {
    inner: UseSharedState<CurrentRoom>,
}

impl UseListenPaymentState {
    pub fn initialize(&self) {}
}
