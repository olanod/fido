use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;
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

#[allow(clippy::needless_return)]
pub fn use_listen_message(cx: &ScopeState) -> &UseListenMessagesState {
    let i18 = use_i18(cx);
    let client = use_client(cx).get();
    let notification = use_notification(cx);
    let session = use_session(cx);
    let room = use_room(cx);
    let messages = use_messages(cx);

    let handler_added = use_ref(cx, || false);

    let key_common_error_thread_id = translate!(i18, "chat.common.error.thread_id");
    let key_common_error_event_id = translate!(i18, "chat.common.error.event_id");
    let key_common_error_user_id = translate!(i18, "chat.common.error.user_id");
    let key_listen_message_image = translate!(i18, "chat.listen.message.image");
    let key_listen_message_file = translate!(i18, "chat.listen.message.file");
    let key_listen_message_video = translate!(i18, "chat.listen.message.video");
    let key_listen_message_html = translate!(i18, "chat.listen.message.html");
    let key_listen_message_thread = translate!(i18, "chat.listen.message.thread");

    let message_dispatch_id =
        use_shared_state::<MessageDispatchId>(cx).expect("Unable to use MessageDispatchId");
    let threading_to = use_thread(cx);

    let task_sender = use_coroutine(
        cx,
        |mut rx: UnboundedReceiver<(MessageEvent, Option<usize>)>| {
            to_owned![
                client,
                messages,
                notification,
                room,
                threading_to,
                session,
                key_common_error_user_id
            ];

            async move {
                while let Some((message_event, message_position_local)) = rx.next().await {
                    if let Some(message) = message_event.mgs {
                        let mut msgs = messages.get().clone();
                        let mut plain_message = None;

                        let is_in_current_room =
                            message_event.room.room_id().as_str().eq(&room.get().id);

                        let last_message_id = messages.get().len() as i64;

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
                                        let relation =
                                            TimelineRelation::CustomThread(TimelineThread {
                                                event_id: timeline_thread.event_id.clone(),
                                                thread: timeline_thread.thread.clone(),
                                                latest_event: match timeline_thread.thread
                                                    [timeline_thread.thread.len() - 1]
                                                    .clone()
                                                    .event_id
                                                {
                                                    Some(id) => id,
                                                    None => {
                                                        notification.handle_error(
                                                            &key_common_error_thread_id,
                                                        );
                                                        return;
                                                    }
                                                },
                                                count: timeline_thread.thread.len(),
                                            });

                                        if is_in_current_room {
                                            msgs.push(relation);
                                        }
                                    }
                                }

                                plain_message = Some(key_listen_message_thread.as_str());
                            }
                            TimelineRelation::None(timeline_message) => {
                                // Position of a head thread timeline
                                let position = msgs.iter().position(|m| {
                                    let TimelineRelation::CustomThread(t) = m else {
                                        return false;
                                    };

                                    let Some(ref id) = timeline_message.event_id else {
                                        notification.handle_error(&key_common_error_event_id);
                                        return false;
                                    };

                                    t.event_id.eq(id)
                                });

                                match position {
                                    Some(p) => {
                                        if let TimelineRelation::CustomThread(ref mut z) = msgs[p] {
                                        }
                                    }
                                    None => {
                                        if is_in_current_room {
                                            let Some(position) = message_position_local else {
                                                msgs.push(message.clone());

                                                return;
                                            };

                                            msgs[position] = message.clone()
                                        } else {
                                            plain_message = Some(message_to_plain_content(
                                                &timeline_message.body,
                                                &key_listen_message_image,
                                                &key_listen_message_file,
                                                &key_listen_message_video,
                                                &key_listen_message_html,
                                            ));
                                        }
                                    }
                                }
                            }
                            TimelineRelation::Reply(timeline_message) => {
                                if is_in_current_room {
                                    let Some(position) = message_position_local else {
                                        msgs.push(message.clone());

                                        return;
                                    };

                                    msgs[position] = message.clone()
                                } else {
                                    plain_message = Some(message_to_plain_content(
                                        &timeline_message.event.body,
                                        &key_listen_message_image,
                                        &key_listen_message_file,
                                        &key_listen_message_video,
                                        &key_listen_message_html,
                                    ));
                                }
                            }
                            TimelineRelation::CustomThread(timeline_message) => {
                                if is_in_current_room {
                                    msgs.push(message);
                                }

                                plain_message = Some(key_listen_message_thread.as_str());
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
                                    if let Some(event_id) = &t.event_id {
                                        if event_id.eq(&thread.event_id) {
                                            let mut new_thread = thread.clone();

                                            new_thread.thread.push(t.clone());

                                            threading_to.set(Some(TimelineThread {
                                                event_id: event_id.clone(),
                                                thread: new_thread.thread.clone(),
                                                count: new_thread.count.clone(),
                                                latest_event: new_thread.latest_event.clone(),
                                            }));
                                        }
                                    };
                                }
                            });
                        }

                        let room_name = match message_event.room.name() {
                            Some(name) => name,
                            None => {
                                let mut name = String::from("Unknown name room");

                                let Some(session_data) = session.get() else {
                                    notification.handle_error(&key_common_error_user_id);
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
            messages,
            session,
            notification,
            key_common_error_user_id
        ];

        async move {
            client.sync_once(SyncSettings::default()).await;

            let Some(me) = session.get() else {
                notification.handle_error(&key_common_error_user_id);
                return;
            };

            if !*handler_added.read() {
                client.add_event_handler(
                    move |ev: OriginalSyncRoomMessageEvent,
                          room: Room,
                          client: matrix_sdk::Client| {
                        let task_sender = task_sender.clone();
                        let messages = messages.clone();
                        let me = me.clone();
                        let mut back_messages = messages.get().clone();

                        let value = &message_dispatch_id.read().value;
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

                        async move {
                            let message_type = &ev.content.msgtype;
                            let event_id = ev.event_id;
                            let Ok(member) = room_member(ev.sender, &room).await else {
                                return;
                            };
                            let relates = &ev.content.relates_to;
                            let time = ev.origin_server_ts;
                            let to_find = to_find.clone();
                            let mut position = None;

                            info!("to find {:?}", to_find);
                            info!("back messages {:?}", back_messages);

                            if let Some((uuid, event_id)) = to_find {
                                position = back_messages.iter().position(|m| match m {
                                    TimelineRelation::None(relation) => {
                                        relation.event_id.as_ref() == Some(&uuid)
                                    }
                                    TimelineRelation::Reply(relation) => {
                                        relation.event.event_id.as_ref() == Some(&uuid)
                                    }
                                    TimelineRelation::CustomThread(relation) => relation
                                        .thread
                                        .iter()
                                        .any(|rm| rm.event_id.as_ref() == Some(&uuid)),
                                    TimelineRelation::Thread(relation) => relation
                                        .thread
                                        .iter()
                                        .any(|rm| rm.event_id.as_ref() == Some(&uuid)),
                                });

                                info!("position {:?}", position);
                            }

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
                            task_sender.send((
                                MessageEvent {
                                    room,
                                    mgs: message_result,
                                },
                                position,
                            ));
                        }
                    },
                );

                handler_added.set(true);
            }

            let _ = client.sync(SyncSettings::default()).await;
        }
    });

    cx.use_hook(move || UseListenMessagesState {})
}

#[derive(Clone)]
pub struct UseListenMessagesState {}

impl UseListenMessagesState {
    pub fn initialize(&self) {}
}

pub fn message_to_plain_content<'a>(
    content: &'a TimelineMessageType,
    key_image: &'a str,
    key_file: &'a str,
    key_video: &'a str,
    key_html: &'a str,
) -> &'a str {
    match &content {
        TimelineMessageType::Image(_) => key_image,
        TimelineMessageType::Text(t) => t,
        TimelineMessageType::File(t) => key_file,
        TimelineMessageType::Video(t) => key_video,
        TimelineMessageType::Html(t) => key_html,
    }
}
