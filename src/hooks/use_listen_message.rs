use std::ops::{Deref, Index};

use dioxus::prelude::*;
use futures_util::StreamExt;
use log::info;
use matrix_sdk::{
    config::SyncSettings, room::Room, ruma::events::room::message::OriginalSyncRoomMessageEvent,
};
use ruma::events::room::message::Relation;

use crate::{
    components::{
        atoms::message::Messages, molecules::rooms::CurrentRoom,
        organisms::chat::utils::handle_notification,
    },
    hooks::use_notification::{NotificationHandle, NotificationItem, NotificationType},
    pages::chat::chat::MessageEvent,
    services::matrix::matrix::{
        format_original_any_room_message_event, format_relation_from_event, room_member,
        TimelineMessage, TimelineMessageType, TimelineRelation, TimelineThread,
    },
};

use super::{
    use_client::use_client, use_init_app::MessageDispatchId, use_notification::use_notification,
    use_session::use_session,
};

#[allow(clippy::needless_return)]
pub fn use_listen_message(cx: &ScopeState) -> &UseListenMessagesState {
    let client = use_client(cx).get();
    let notification = use_notification(cx);

    let handler_added = use_ref(cx, || false);

    let message_dispatch_id =
        use_shared_state::<MessageDispatchId>(cx).expect("Unable to use MessageDispatchId");
    let messages = use_shared_state::<Messages>(cx).expect("Unable to use Messages");
    let current_room = use_shared_state::<CurrentRoom>(cx).expect("Unable to use CurrentRoom");
    let timeline_thread =
        use_shared_state::<Option<TimelineThread>>(cx).expect("Unable to use TimelineThread");

    let task_sender = use_coroutine(
        cx,
        |mut rx: UnboundedReceiver<(MessageEvent, Option<usize>)>| {
            to_owned![
                client,
                messages,
                notification,
                current_room,
                timeline_thread,
                session
            ];

            async move {
                while let Some((message_event, message_position_local)) = rx.next().await {
                    if let Some(message) = message_event.mgs {
                        let mut msgs = messages.read().clone();
                        let mut plain_message = None;

                        let is_in_current_room = message_event
                            .room
                            .room_id()
                            .as_str()
                            .eq(&current_room.read().id);

                        let last_message_id = messages.read().len() as i64;

                        match &message {
                            TimelineRelation::Thread(timeline_thread) => {
                                // Position of an existing thread timeline
                                info!("current on thread listen");

                                let position = msgs.iter().position(|m| {
                                    if let TimelineRelation::CustomThread(t) = m {
                                        t.event_id.eq(&timeline_thread.event_id)
                                    } else {
                                        false
                                    }
                                });

                                info!("current position on thread listen {:?}", position);

                                if let Some(p) = position {
                                    if let TimelineRelation::CustomThread(ref mut t) = msgs[p] {
                                        t.thread.push(timeline_thread.thread[0].clone());
                                    };
                                } else {
                                    let relation = TimelineRelation::CustomThread(TimelineThread {
                                        event_id: timeline_thread.event_id.clone(),
                                        thread: timeline_thread.thread.clone(),
                                        latest_event: match timeline_thread.thread
                                            [timeline_thread.thread.len() - 1]
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
                                        count: timeline_thread.thread.len(),
                                    });

                                    if is_in_current_room {
                                        msgs.push(relation);
                                    }
                                }

                                plain_message = Some("Nuevo mensaje en el hilo");
                            }
                            TimelineRelation::None(x) => {
                                // Position of a head thread timeline
                                let position = msgs.iter().position(|m| {
                                    if let TimelineRelation::CustomThread(t) = m {
                                        match &x.event_id {
                                            Some(id) => t.event_id.eq(id),
                                            None => {
                                                notification.handle_error(
                                                    "Error inesperado: (Id de evento)",
                                                );
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
                                        match message_position_local {
                                            Some(position) => msgs[position] = message.clone(),
                                            None => {
                                                msgs.push(message.clone());

                                                plain_message =
                                                    Some(message_to_plain_content(&x.body))
                                            }
                                        }
                                    }
                                }
                            }
                            TimelineRelation::Reply(x) => {
                                if is_in_current_room {
                                    match message_position_local {
                                        Some(position) => msgs[position] = message.clone(),
                                        None => {
                                            msgs.push(message.clone());
                                            plain_message =
                                                Some(message_to_plain_content(&x.event.body))
                                        }
                                    }
                                }
                            }
                            TimelineRelation::CustomThread(x) => {
                                info!("current on custom thread listen");
                                if is_in_current_room {
                                    msgs.push(message);
                                }

                                plain_message = Some("Nuevo mensaje en el hilo");
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

                        let room_name = match message_event.room.name() {
                            Some(name) => name,
                            None => {
                                let mut name = String::from("Unknown name room");
                                let session_data = match session.get() {
                                    Some(data) => data,
                                    None => {
                                        notification.set(NotificationItem {
                                            title: String::from(""),
                                            body: String::from(""),
                                            show: false,
                                            handle: NotificationHandle {
                                                value: NotificationType::None,
                                            },
                                        });

                                        return;
                                    }
                                };
                                let users = message_event.room.members().await;

                                if let Ok(members) = users {
                                    let member = members
                                        .into_iter()
                                        .find(|member| !member.user_id().eq(&session_data.user_id));

                                    if let Some(m) = member {
                                        let n = m.name();

                                        name = String::from(n);
                                    }
                                }

                                name
                            }
                        };

                        if let Some(content) = plain_message {
                            handle_notification(
                                NotificationItem {
                                    title: String::from(room_name),
                                    body: String::from(content),
                                    show: true,
                                    handle: NotificationHandle {
                                        value: NotificationType::Click,
                                    },
                                },
                                notification.to_owned(),
                            );
                        }
                    }
                }
            }
        },
    )
    .clone();

    // After logging is mandatory to perform a client sync,
    // since the chat needs sync to listen for new messages
    // this coroutine is necesary
    use_coroutine(cx, |_: UnboundedReceiver<String>| {
        to_owned![
            client,
            handler_added,
            task_sender,
            message_dispatch_id,
            messages
        ];

        async move {
            client.sync_once(SyncSettings::default()).await;

            if !*handler_added.read() {
                client.add_event_handler(
                    move |ev: OriginalSyncRoomMessageEvent,
                          room: Room,
                          client: matrix_sdk::Client| {
                        let task_sender = task_sender.clone();
                        let messages = messages.clone();

                        let user = client.user_id();

                        let me = match user {
                            Some(u) => u.to_string(),
                            None => {
                                panic!("User not found");
                            }
                        };

                        let value = &message_dispatch_id.read().value;
                        let to_find: Option<(String, Option<String>)> =
                            value.iter().find_map(|v| {
                                let x = &v.1.clone();

                                let x = match x {
                                    Some(x) => {
                                        if ev.event_id.eq(x) {
                                            Some((v.0.clone(), v.1.clone()))
                                        } else {
                                            None
                                        }
                                    }
                                    None => None,
                                };
                                x
                            });

                        // info!(
                        //     "message_dispatch_id listen message  {:#?}",
                        //     *message_dispatch_id.read()
                        // );

                        let mut back_messages = messages.read().clone();
                        async move {
                            let message_type = &ev.content.msgtype;
                            let event_id = ev.event_id;
                            let member = room_member(ev.sender, &room).await;
                            let relates = &ev.content.relates_to;
                            let time = ev.origin_server_ts;
                            let to_find = to_find.clone();
                            let mut position = None;

                            info!("to find {:?}", to_find);
                            info!("back messages {:?}", back_messages);

                            if let Some((uuid, event_id)) = to_find {
                                position = back_messages.iter().position(|m| match m {
                                    TimelineRelation::None(relation) => {
                                        relation.event_id.eq(&Some(uuid.clone()))
                                    }
                                    TimelineRelation::Reply(relation) => {
                                        relation.event.clone().event_id.eq(&Some(uuid.clone()))
                                    }
                                    TimelineRelation::CustomThread(relation) => {
                                        info!("into customthread listen");
                                        let position = relation
                                            .thread
                                            .iter()
                                            .position(|rm| rm.event_id.eq(&Some(uuid.clone())));

                                        match position {
                                            Some(_) => true,
                                            None => false,
                                        }
                                    }
                                    TimelineRelation::Thread(relation) => {
                                        let position = relation
                                            .thread
                                            .iter()
                                            .position(|rm| rm.event_id.eq(&Some(uuid.clone())));

                                        match position {
                                            Some(_) => true,
                                            None => false,
                                        }
                                    }
                                });

                                info!("position {:?}", position);
                            }

                            let formatted_message = format_original_any_room_message_event(
                                &message_type,
                                event_id,
                                &member,
                                &me,
                                time,
                                &client,
                            )
                            .await;

                            let mut message_result = None;

                            match relates {
                                Some(relation) => match &relation {
                                    Relation::_Custom => {
                                        if let Some(x) = formatted_message {
                                            message_result = Some(TimelineRelation::None(x));
                                        }
                                    }

                                    _ => {
                                        if let Some(x) = formatted_message {
                                            message_result = format_relation_from_event(
                                                &message_type,
                                                relates,
                                                &room,
                                                x,
                                                &member,
                                                &me,
                                                time,
                                                &client,
                                            )
                                            .await;
                                        }
                                    }
                                },
                                None => {
                                    if let Some(x) = formatted_message {
                                        message_result = Some(TimelineRelation::None(x));
                                    }
                                }
                            }

                            task_sender.send((
                                MessageEvent {
                                    room,
                                    mgs: message_result,
                                },
                                position,
                            ))
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

pub fn message_to_plain_content(content: &TimelineMessageType) -> &str {
    match &content {
        TimelineMessageType::Image(_) => "Imagen",
        TimelineMessageType::Text(t) => t,
        TimelineMessageType::File(t) => "Archivo adjunto",
        TimelineMessageType::Video(t) => "Video",
        TimelineMessageType::Html(t) => "Bloque de texto",
    }
}
