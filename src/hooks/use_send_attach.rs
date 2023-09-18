use dioxus::prelude::*;
use futures_util::StreamExt;
use matrix_sdk::ruma::RoomId;

use crate::{
    components::molecules::input_message::ReplyingTo,
    services::matrix::matrix::{send_attachment, Attachment},
};

use super::{use_client::use_client, use_room::use_room};

#[allow(clippy::needless_return)]
pub fn use_send_attach(cx: &ScopeState) -> &UseSendMessageState {
    let client = use_client(cx).get();
    let room = use_room(cx).get();
    let _replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();

    let task_push_attach = use_coroutine(cx, |mut rx: UnboundedReceiver<Vec<u8>>| {
        to_owned![client];

        async move {
            while let Some(message_item) = rx.next().await {
                let room_id = RoomId::parse(room.id.clone()).unwrap();
                send_attachment(&client, &room_id, &Attachment { data: message_item }).await;
            }
        }
    });

    cx.use_hook(move || UseSendMessageState {
        inner: task_push_attach.clone(),
    })
}

#[derive(Clone)]
pub struct UseSendMessageState {
    inner: Coroutine<Vec<u8>>,
}

impl UseSendMessageState {
    pub fn send(&self, message: Vec<u8>) {
        self.inner.send(message)
    }
}
