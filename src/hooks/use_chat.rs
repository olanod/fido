use std::{collections::HashMap, ops::Deref};

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;
use log::info;
use matrix_sdk::ruma::RoomId;

use crate::{
    components::atoms::message::Messages,
    hooks::use_notification::{NotificationHandle, NotificationItem, NotificationType},
    services::matrix::matrix::{timeline, TimelineRelation, TimelineThread},
};

use super::{
    use_client::use_client, use_messages::use_messages, use_notification::use_notification,
    use_room::use_room, use_session::use_session,
};

#[allow(clippy::needless_return)]
pub fn use_chat(cx: &ScopeState) -> &UseChatState {
    let i18 = use_i18(cx);
    let client = use_client(cx).get();
    let session = use_session(cx);
    let notification = use_notification(cx);
    let room = use_room(cx);
    let mmessages = use_messages(cx);

    let key_common_error_room_id = translate!(i18, "chat.common.error.room_id");

    let mutable_messages = use_ref::<Messages>(cx, || vec![]);
    let messages_loading = use_ref::<bool>(cx, || false);
    let limit_events_by_room = use_ref::<HashMap<String, u64>>(cx, || HashMap::new());
    let from = use_ref::<Option<String>>(cx, || None);
    let timeline_thread =
        use_shared_state::<Option<TimelineThread>>(cx).expect("Unable to use TimelineThread");

    let task_timeline = use_coroutine(cx, |mut rx: UnboundedReceiver<bool>| {
        to_owned![
            client,
            room,
            mmessages,
            mutable_messages,
            messages_loading,
            limit_events_by_room,
            from,
            timeline_thread,
            session,
            notification
        ];

        async move {
            while let Some(true) = rx.next().await {
                messages_loading.set(true);

                let current_room_id = room.get().id.clone();
                let current_events = match limit_events_by_room.read().get(&current_room_id) {
                    Some(c) => c,
                    None => &(15 as u64),
                }
                .clone();

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

                let room_id = match RoomId::parse(&current_room_id) {
                    Ok(id) => id,
                    Err(_) => {
                        notification.handle_error("{key_common_error_room_id}");
                        return;
                    }
                };
                let ms = mmessages.get().clone();

                let (f, msg) = timeline(
                    &client,
                    &room_id,
                    current_events,
                    from.read().clone(),
                    ms.to_vec(),
                    session_data,
                )
                .await;

                from.set(f);

                info!("before write xxx");
                mmessages.set(msg);
                info!("after write xxx");
                let mm = timeline_thread.read().clone();

                if let Some(thread) = mm {
                    let ms = mmessages.get().clone();
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
                        } else {
                            false
                        }
                    });
                }

                messages_loading.set(false);
            }
        }
    });

    use_effect(cx, (limit_events_by_room,), |(_,)| {
        to_owned![task_timeline];
        async move {
            task_timeline.send(true);
        }
    });

    cx.use_hook(move || UseChatState {
        inner: ChatState {
            messages: mmessages.get().clone(),
            isLoading: messages_loading.clone(),
            limit: limit_events_by_room.clone(),
            task: task_timeline.clone(),
        },
    })
}

#[derive(Clone)]
pub struct ChatState {
    messages: Vec<TimelineRelation>,
    isLoading: UseRef<bool>,
    limit: UseRef<HashMap<String, u64>>,
    task: Coroutine<bool>,
}

#[derive(Clone)]
pub struct UseChat {
    pub messages: Messages,
    pub isLoading: bool,
    pub limit: HashMap<String, u64>,
    pub task: Coroutine<bool>,
}

#[derive(Clone)]
pub struct UseChatState {
    inner: ChatState,
}

impl UseChatState {
    pub fn get(&self) -> UseChat {
        let inner = &self.inner;

        UseChat {
            messages: inner.messages.clone(),
            isLoading: *inner.isLoading.read(),
            limit: inner.limit.read().deref().clone(),
            task: inner.task.clone(),
        }
    }

    pub fn get_mut(&self) -> ChatState {
        self.inner.clone()
    }

    pub fn set(&self, state: ChatState) {
        let mut inner = &self.inner;
        inner = &state;
    }

    pub fn loadmore(&self, current_room_id: &str) {
        let current_events = match self.inner.limit.read().get(current_room_id) {
            Some(c) => c.clone(),
            None => 15 as u64,
        };

        self.inner
            .limit
            .with_mut(|lr| lr.insert(current_room_id.to_string(), current_events + 5));
    }
}
