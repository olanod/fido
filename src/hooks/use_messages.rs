use std::{collections::HashMap, ops::Deref};

use dioxus::prelude::*;
use futures_util::StreamExt;
use log::info;
use matrix_sdk::ruma::RoomId;

use crate::{
    components::{atoms::message::Messages, molecules::rooms::CurrentRoom},
    services::matrix::matrix::{timeline, TimelineRelation, TimelineThread},
};

use super::use_client::use_client;

#[allow(clippy::needless_return)]
pub fn use_messages(cx: &ScopeState) -> &UseMessagesState {
    let client = use_client(cx).get();
    let current_room = use_shared_state::<CurrentRoom>(cx).unwrap();
    let messages = use_shared_state::<Messages>(cx).expect("Messages not provided");
    let mutable_messages = use_ref::<Messages>(cx, || vec![]);
    let messages_loading = use_ref::<bool>(cx, || false);
    let limit_events_by_room = use_ref::<HashMap<String, u64>>(cx, || HashMap::new());
    let from = use_ref::<Option<String>>(cx, || None);
    let timeline_thread = use_shared_state::<Option<TimelineThread>>(cx).unwrap();

    let task_timeline = use_coroutine(cx, |mut rx: UnboundedReceiver<bool>| {
        to_owned![
            client,
            current_room,
            messages,
            mutable_messages,
            messages_loading,
            limit_events_by_room,
            from,
            timeline_thread
        ];

        async move {
            while let Some(true) = rx.next().await {
                messages_loading.set(true);

                let current_room_id = current_room.read().id.clone();
                let current_events = limit_events_by_room
                    .read()
                    .get(&current_room_id)
                    .unwrap_or_else(|| &(15 as u64))
                    .to_owned();

                let room_id = RoomId::parse(current_room.read().id.clone()).unwrap();
                let ms = messages.read().deref().clone();

                let (f, msg) =
                    timeline(&client, &room_id, current_events, from.read().clone(), ms).await;

                from.set(f);

                info!("before write xxx");
                *messages.write() = msg;
                info!("after write xxx");
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

    cx.use_hook(move || UseMessagesState {
        inner: MessagesState {
            messages: messages.clone(),
            isLoading: messages_loading.clone(),
            limit: limit_events_by_room.clone(),
            task: task_timeline.clone(),
        },
    })
}

#[derive(Clone)]
pub struct MessagesState {
    messages: UseSharedState<Messages>,
    isLoading: UseRef<bool>,
    limit: UseRef<HashMap<String, u64>>,
    task: Coroutine<bool>,
}

#[derive(Clone)]
pub struct UseMessages {
    pub messages: Messages,
    pub isLoading: bool,
    pub limit: HashMap<String, u64>,
    pub task: Coroutine<bool>,
}

#[derive(Clone)]
pub struct UseMessagesState {
    inner: MessagesState,
}

impl UseMessagesState {
    pub fn get(&self) -> UseMessages {
        let inner = &self.inner;

        let binding = inner.messages.read();
        let messages = binding.deref();

        UseMessages {
            messages: messages.clone(),
            isLoading: *inner.isLoading.read(),
            limit: inner.limit.read().deref().clone(),
            task: inner.task.clone(),
        }
    }

    pub fn get_mut(&self) -> MessagesState {
        self.inner.clone()
    }

    pub fn set(&self, state: MessagesState) {
        let mut inner = &self.inner;
        inner = &state;
    }

    pub fn set_messages(&self, messages: Messages) {
        let mut inner = self.inner.messages.write();
        *inner = messages;
    }

    pub fn loadmore(&self, current_room_id: String) {
        let current_events = *self
            .inner
            .limit
            .read()
            .get(&current_room_id)
            .unwrap_or_else(|| &(15 as u64));

        self.inner
            .limit
            .with_mut(|lr| lr.insert(current_room_id, current_events + 5));
    }

    pub fn push(&self, message: TimelineRelation) {
        self.inner.messages.write().push(message);
    }
}
