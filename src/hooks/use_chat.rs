use std::{collections::HashMap, ops::Deref};

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;
use matrix_sdk::ruma::RoomId;

use crate::{
    components::atoms::message::Messages,
    services::matrix::matrix::{timeline, TimelineError, TimelineRelation, TimelineThread},
};

use super::{
    use_client::{use_client, UseClientState},
    use_messages::{use_messages, UseMessagesState},
    use_notification::use_notification,
    use_room::use_room,
    use_session::{use_session, UseSessionState},
    use_thread::use_thread,
};

pub enum ChatError {
    InvalidSession,
    InvalidRoom,
    TimelineError(TimelineError),
}

pub fn use_chat() -> UseChatState {
    let i18 = use_i18();
    let client = use_client();
    let session = use_session();
    let mut notification = use_notification();
    let room = use_room();
    let mut messages = use_messages();
    let mut threading_to = use_thread();

    let mut messages_loading = use_signal::<bool>(|| false);
    let limit_events_by_room = use_signal::<HashMap<String, u64>>(|| HashMap::new());
    let mut from = use_signal::<Option<String>>(|| None);

    let task_timeline = use_coroutine(|mut rx: UnboundedReceiver<()>| async move {
        while let Some(_) = rx.next().await {
            messages_loading.set(true);

            let current_room_id = room.get().id.clone();
            let current_events = limit_events_by_room
                .read()
                .get(&current_room_id)
                .unwrap_or(&15)
                .clone();

            if let Err(e) = process(
                current_events,
                &session,
                &mut messages,
                &client,
                &mut from,
                &current_room_id,
            )
            .await
            {
                let message = match e {
                    ChatError::InvalidSession => translate!(i18, "chat.session.error.not_found"),
                    ChatError::InvalidRoom => translate!(i18, "chat.common.error.room_id"),
                    ChatError::TimelineError(TimelineError::RoomNotFound) => {
                        translate!(i18, "chat.message_list.errors.room_not_found")
                    }
                    ChatError::TimelineError(TimelineError::InvalidLimit) => {
                        translate!(i18, "chat.message_list.errors.timeline_invalid_limit")
                    }
                    ChatError::TimelineError(TimelineError::MessagesNotFound) => {
                        translate!(i18, "chat.message_list.errors.timeline_not_found")
                    }
                };

                messages_loading.set(false);
                notification.handle_error(&message);
            };

            if let Some(thread) = threading_to.get() {
                messages.get().iter().find_map(|m| {
                    let TimelineRelation::CustomThread(t) = m else {
                        return None;
                    };

                    if t.event_id.eq(&thread.event_id) {
                        threading_to.set(Some(TimelineThread {
                            event_id: t.event_id.clone(),
                            thread: t.thread.clone(),
                            count: t.count.clone(),
                            latest_event: t.latest_event.clone(),
                        }));

                        return Some(());
                    }

                    None
                });
            }

            messages_loading.set(false);
        }
    });

    use_effect(use_reactive((&limit_events_by_room,), move |(_,)| {
        task_timeline.send(());
    }));

    use_effect(use_reactive((&room.get().id,), move |(_,)| {
        from.set(None);
        task_timeline.send(());
    }));

    use_hook(move || UseChatState {
        inner: ChatState {
            messages: messages.get().clone(),
            isLoading: messages_loading.clone(),
            limit: limit_events_by_room.clone(),
            task: task_timeline.clone(),
        },
    })
}

#[derive(Clone)]
pub struct ChatState {
    messages: Vec<TimelineRelation>,
    isLoading: Signal<bool>,
    limit: Signal<HashMap<String, u64>>,
    task: Coroutine<()>,
}

#[derive(Clone)]
pub struct UseChat {
    pub messages: Messages,
    pub isLoading: bool,
    pub limit: HashMap<String, u64>,
    pub task: Coroutine<()>,
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

    pub fn set(&mut self, state: ChatState) {
        self.inner = state;
    }

    pub fn loadmore(&mut self, current_room_id: &str) {
        let current_events = self
            .inner
            .limit
            .read()
            .get(current_room_id)
            .unwrap_or(&(15 as u64))
            .clone();

        self.inner
            .limit
            .with_mut(|lr| lr.insert(current_room_id.to_string(), current_events + 5));
    }
}

async fn process(
    current_events: u64,
    session: &UseSessionState,
    messages: &mut UseMessagesState,
    client: &UseClientState,
    from: &mut Signal<Option<String>>,
    current_room_id: &str,
) -> Result<(), ChatError> {
    let session_data = session.get().ok_or(ChatError::InvalidSession)?;
    let room_id = RoomId::parse(&current_room_id).map_err(|_| ChatError::InvalidRoom)?;

    let (f, msg) = timeline(
        &client.get(),
        &room_id,
        current_events,
        from.read().clone(),
        messages.get().clone().to_vec(),
        session_data,
    )
    .await
    .map_err(|e| ChatError::TimelineError(e))?;

    from.set(f);
    messages.set(msg);

    Ok(())
}
