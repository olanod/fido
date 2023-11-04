use dioxus::prelude::*;
use futures_util::StreamExt;
use matrix_sdk::ruma::{EventId, RoomId};

use crate::{
    components::molecules::input_message::ReplyingTo,
    services::matrix::matrix::{send_attachment, Attachment, TimelineMessageThread, TimelineThread},
};

use super::{use_client::use_client, use_room::use_room};

#[allow(clippy::needless_return)]
pub fn use_send_attach(cx: &ScopeState) -> &UseSendMessageState {
    let client = use_client(cx).get();
    let room = use_room(cx).get();
    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();
    let threading_to =
        use_shared_state::<Option<TimelineThread>>(cx).expect("Cannot found thread_to");

    let task_push_attach = use_coroutine(cx, |mut rx: UnboundedReceiver<Attachment>| {
        to_owned![client, replying_to, threading_to];

        async move {
            while let Some(message_item) = rx.next().await {
                let room_id = RoomId::parse(room.id.clone()).unwrap();
                let reply_to = replying_to.read().clone();
                let thread_to = threading_to.read().clone();

                let reply_event_id = if let Some(e) = reply_to {
                    Some(EventId::parse(e.event_id).unwrap())
                } else {
                    None
                };

                let thread_event_id = if let Some(e) = &thread_to {
                    Some(EventId::parse(e.event_id.clone()).unwrap())
                } else {
                    None
                };

                let latest_event_id = if let Some(e) = thread_to {
                    let x = e.latest_event;
                    Some(EventId::parse(x).unwrap())
                } else {
                    None
                };

                send_attachment(
                    &client,
                    &room_id,
                    &message_item,
                    reply_event_id,
                    thread_event_id,
                    latest_event_id,
                )
                .await;
            }
        }
    });

    cx.use_hook(move || UseSendMessageState {
        inner: task_push_attach.clone(),
    })
}

#[derive(Clone)]
pub struct UseSendMessageState {
    inner: Coroutine<Attachment>,
}

impl UseSendMessageState {
    pub fn send(&self, message: Attachment) {
        self.inner.send(message)
    }
}
