use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::{StreamExt, TryFutureExt};
use log::info;
use matrix_sdk::{
    config::SyncSettings, room::Room, ruma::events::room::message::OriginalSyncRoomMessageEvent,
};
use ruma::events::room::message::Relation;

use crate::{
    hooks::use_notification::{NotificationHandle, NotificationItem, NotificationType},
    pages::chat::chat::MessageEvent,
    services::matrix::matrix::{
        format_original_any_room_message_event, format_relation_from_event, room_member,
        TimelineMessageType, TimelineRelation, TimelineThread,
    },
};

use super::{
    use_client::use_client, use_init_app::MessageDispatchId, use_messages::use_messages,
    use_notification::use_notification, use_room::use_room, use_session::use_session,
    use_thread::use_thread,
};

pub fn use_listen_message() -> UseListenMessagesState {
    let i18 = use_i18();
    let client = use_client().get();
    let mut notification = use_notification();
    let session = use_session();
    let room = use_room();
    let mut messages = use_messages();

    let mut handler_added = use_signal(|| false);

    let message_dispatch_id = consume_context::<Signal<MessageDispatchId>>();
    let mut threading_to = use_thread();
    let mut position = use_signal::<Option<usize>>(|| None);

    let task_sender = use_coroutine(|mut rx: UnboundedReceiver<MessageEvent>| {
        async move {
            while let Some(message_event) = rx.next().await {
                let message_position_local = *position.read();
                if let Some(message) = message_event.mgs {
                    let mut msgs = messages.get().clone();
                    let mut plain_message: Option<String> = None;

                    let is_in_current_room =
                        message_event.room.room_id().as_str().eq(&room.get().id);

                    match &message {
                        TimelineRelation::Thread(timeline_thread) => {
                            // Position of an existing thread timeline
                            let position = msgs.iter().position(|m| {
                                let TimelineRelation::CustomThread(t) = m else {
                                    return false;
                                };

                                t.event_id.eq(&timeline_thread.event_id)
                            });

                            match position {
                                Some(p) => {
                                    if let TimelineRelation::CustomThread(ref mut t) = msgs[p] {
                                        t.thread.push(timeline_thread.thread[0].clone());
                                    };
                                }
                                None => {
                                    let relation = TimelineRelation::CustomThread(TimelineThread {
                                        event_id: timeline_thread.event_id.clone(),
                                        thread: timeline_thread.thread.clone(),
                                        latest_event: timeline_thread.thread
                                            [timeline_thread.thread.len() - 1]
                                            .clone()
                                            .event_id,
                                        count: timeline_thread.thread.len(),
                                    });

                                    if is_in_current_room {
                                        msgs.push(relation);
                                    }
                                }
                            }

                            plain_message = Some(translate!(i18, "chat.listen.message.thread"));
                        }
                        TimelineRelation::None(timeline_message) => {
                            // Position of a head thread timeline
                            let position = msgs.iter().position(|m| {
                                let TimelineRelation::CustomThread(t) = m else {
                                    return false;
                                };

                                t.event_id.eq(&timeline_message.event_id)
                            });

                            match position {
                                Some(p) => if let TimelineRelation::CustomThread(_) = msgs[p] {},
                                None => {
                                    if is_in_current_room {
                                        if let Some(position) = message_position_local {
                                            msgs[position] = message.clone()
                                        } else {
                                            msgs.push(message.clone());
                                        }
                                    } else {
                                        plain_message = Some(message_to_plain_content(
                                            &timeline_message.body.clone(),
                                            &translate!(i18, "chat.listen.message.image"),
                                            &translate!(i18, "chat.listen.message.file"),
                                            &translate!(i18, "chat.listen.message.video"),
                                            &translate!(i18, "chat.listen.message.html"),
                                        ));
                                    }
                                }
                            }
                        }
                        TimelineRelation::Reply(timeline_message) => {
                            if is_in_current_room {
                                if let Some(position) = message_position_local {
                                    msgs[position] = message.clone();
                                } else {
                                    msgs.push(message.clone());
                                };
                            } else {
                                plain_message = Some(message_to_plain_content(
                                    &timeline_message.event.body.clone(),
                                    &translate!(i18, "chat.listen.message.image"),
                                    &translate!(i18, "chat.listen.message.file"),
                                    &translate!(i18, "chat.listen.message.video"),
                                    &translate!(i18, "chat.listen.message.html"),
                                ));
                            }
                        }
                        TimelineRelation::CustomThread(_) => {
                            if is_in_current_room {
                                msgs.push(message);
                            }

                            plain_message = Some(translate!(i18, "chat.listen.message.thread"));
                        }
                    };

                    messages.set(msgs.clone());
                    let thread_to = threading_to.get().clone();

                    if let Some(thread) = thread_to {
                        messages.get().iter().for_each(|m| {
                            if let TimelineRelation::CustomThread(t) = m {
                                if t.event_id.eq(&thread.event_id) {
                                    threading_to.set(Some(TimelineThread {
                                        event_id: t.event_id.clone(),
                                        thread: t.thread.clone(),
                                        count: t.count.clone(),
                                        latest_event: t.latest_event.clone(),
                                    }));
                                }
                            } else if let TimelineRelation::None(t) = m {
                                if t.event_id.eq(&thread.event_id) {
                                    let mut new_thread = thread.clone();

                                    new_thread.thread.push(t.clone());

                                    threading_to.set(Some(TimelineThread {
                                        event_id: t.event_id.clone(),
                                        thread: new_thread.thread.clone(),
                                        count: new_thread.count.clone(),
                                        latest_event: new_thread.latest_event.clone(),
                                    }));
                                }
                            }
                        });
                    }

                    let room_name = match message_event.room.name() {
                        Some(name) => name,
                        None => {
                            let mut name = String::from("Unknown name room");

                            let Some(session_data) = session.get() else {
                                notification
                                    .handle_error(&translate!(i18, "chat.common.error.user_id"));
                                return;
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
                        notification.handle_notification(NotificationItem {
                            title: room_name,
                            body: String::from(content),
                            show: true,
                            handle: NotificationHandle {
                                value: NotificationType::Click,
                            },
                        })
                    }
                }
            }
        }
    })
    .clone();

    let task_replacer = use_coroutine(
        |mut rx: UnboundedReceiver<OriginalSyncRoomMessageEvent>| async move {
            while let Some(ev) = rx.next().await {
                let back_messages = messages.get().clone();
                let value = &message_dispatch_id.read().value;
                position.set(None);

                let to_find: Option<(String, Option<String>)> =
                    value.iter().find_map(|(uuid, event_id)| {
                        event_id.clone().and_then(|id| {
                            if ev.event_id == id {
                                Some((uuid.clone(), event_id.clone()))
                            } else {
                                None
                            }
                        })
                    });

                if let Some((uuid, _)) = to_find {
                    position.set(back_messages.iter().position(|m| match m {
                        TimelineRelation::None(relation) => relation.event_id == uuid,
                        TimelineRelation::Reply(relation) => relation.event.event_id == uuid,
                        TimelineRelation::CustomThread(relation) => {
                            relation.thread.iter().any(|rm| rm.event_id == uuid)
                        }
                        TimelineRelation::Thread(relation) => {
                            relation.thread.iter().any(|rm| rm.event_id == uuid)
                        }
                    }));

                    info!("position {:?}", position.read());
                }
            }
        },
    );

    // After logging is mandatory to perform a client sync,
    // since the chat needs sync to listen for new messages
    // this coroutine is necesary
    use_coroutine(|_: UnboundedReceiver<String>| {
        async move {
            let me = session.get().ok_or(ListenMessageError::SessionNotFound)?;

            if !*handler_added.read() {
                client.add_event_handler(
                    move |ev: OriginalSyncRoomMessageEvent,
                          room: Room,
                          client: matrix_sdk::Client| {
                        let task_sender = task_sender.clone();
                        let me = me.clone();
                        task_replacer.send(ev.clone());

                        async move {
                            let message_type = &ev.content.msgtype;
                            let event_id = ev.event_id;
                            let Ok(member) = room_member(ev.sender, &room).await else {
                                return;
                            };
                            let relates = &ev.content.relates_to;
                            let time = ev.origin_server_ts;

                            let formatted_message = format_original_any_room_message_event(
                                &message_type,
                                event_id,
                                &member,
                                &me.user_id,
                                time,
                                &client,
                            )
                            .await;

                            let mut message_result = None;

                            match relates {
                                Some(relation) => match &relation {
                                    Relation::_Custom => {
                                        if let Some(timeline_message) = formatted_message {
                                            message_result =
                                                Some(TimelineRelation::None(timeline_message));
                                        }
                                    }
                                    _ => {
                                        if let Some(timeline_message) = formatted_message {
                                            message_result = format_relation_from_event(
                                                &message_type,
                                                relates,
                                                &room,
                                                timeline_message,
                                                &member,
                                                &me.user_id,
                                                time,
                                                &client,
                                            )
                                            .await;
                                        }
                                    }
                                },
                                None => {
                                    if let Some(timeline_message) = formatted_message {
                                        message_result =
                                            Some(TimelineRelation::None(timeline_message));
                                    }
                                }
                            }
                            task_sender.send(MessageEvent {
                                room,
                                mgs: message_result,
                            });
                        }
                    },
                );

                handler_added.set(true);
            }

            client
                .sync(SyncSettings::default())
                .await
                .map_err(|_| ListenMessageError::FailedSync)?;

            Ok::<(), ListenMessageError>(())
        }
        .unwrap_or_else(move |e: ListenMessageError| {
            let message = match e {
                ListenMessageError::FailedSync => translate!(i18, "chat.common.error.sync"),
                ListenMessageError::SessionNotFound => translate!(i18, "chat.common.error.user_id"),
            };
            notification.handle_error(&message);
        })
    });

    use_hook(move || UseListenMessagesState {})
}

pub enum ListenMessageError {
    FailedSync,
    SessionNotFound,
}

#[derive(Clone, Copy)]
pub struct UseListenMessagesState {}

impl UseListenMessagesState {
    pub fn initialize(&self) {}
}

pub fn message_to_plain_content(
    content: &TimelineMessageType,
    key_image: &str,
    key_file: &str,
    key_video: &str,
    key_html: &str,
) -> String {
    match &content {
        TimelineMessageType::Image(_) => key_image.to_owned(),
        TimelineMessageType::Text(t) => t.to_owned(),
        TimelineMessageType::File(_) => key_file.to_owned(),
        TimelineMessageType::Video(_) => key_video.to_owned(),
        TimelineMessageType::Html(_) => key_html.to_owned(),
    }
}
