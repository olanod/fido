use std::{collections::HashMap, ops::Deref};

use dioxus::prelude::*;
use futures_util::StreamExt;
use matrix_sdk::ruma::RoomId;

use crate::{
    components::{
        atoms::{
            message::{Message, Messages},
            MessageReply,
        },
        molecules::rooms::CurrentRoom,
    },
    services::matrix::matrix::timeline,
};

use super::use_client::use_client;

#[allow(clippy::needless_return)]
pub fn use_messages(cx: &ScopeState) -> &UseMessagesState {
    let client = use_client(cx).get();
    let current_room = use_shared_state::<CurrentRoom>(cx).unwrap();
    let messages = use_shared_state::<Messages>(cx).expect("Matrix client not provided");
    let messages_loading = use_ref::<bool>(cx, || false);
    let limit_events_by_room = use_ref::<HashMap<String, u64>>(cx, || HashMap::new());

    let task_timeline = use_coroutine(cx, |mut rx: UnboundedReceiver<bool>| {
        to_owned![
            client,
            current_room,
            messages,
            messages_loading,
            limit_events_by_room
        ];

        async move {
            while let Some(true) = rx.next().await {
                messages_loading.set(true);
                messages.write().clear();

                let current_room_id = current_room.read().id.clone();
                let current_events = limit_events_by_room
                    .read()
                    .get(&current_room_id)
                    .unwrap_or_else(|| &(15 as u64))
                    .to_owned();

                let room_id = RoomId::parse(current_room.read().id.clone()).unwrap();

                let msg = timeline(&client, &room_id, current_events).await;

                for m in msg.iter() {
                    let mut rep: Option<MessageReply> = None;

                    if let Some(r) = &m.reply {
                        rep = Some(MessageReply {
                            display_name: r.sender.name.clone(),
                            avatar_uri: r.sender.avatar_uri.clone(),
                            content: r.body.clone(),
                        })
                    }

                    let last_message_id = messages.read().len() as i64;
                    messages.write().push(Message {
                        id: last_message_id,
                        event_id: m.event_id.clone(),
                        display_name: m.sender.name.clone(),
                        content: m.body.clone(),
                        avatar_uri: m.sender.avatar_uri.clone(),
                        reply: rep.clone(),
                        origin: m.origin.clone(),
                        time: m.time.clone(),
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
        },
    })
}

#[derive(Clone)]
pub struct MessagesState {
    messages: UseSharedState<Messages>,
    isLoading: UseRef<bool>,
    limit: UseRef<HashMap<String, u64>>,
}

#[derive(Clone)]
pub struct UseMessages {
    pub messages: Messages,
    pub isLoading: bool,
    pub limit: HashMap<String, u64>,
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
            .with_mut(|lr| lr.insert(current_room_id, current_events + 15));
    }
}
