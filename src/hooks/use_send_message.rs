use dioxus::prelude::*;
use futures_util::StreamExt;
use matrix_sdk::ruma::{EventId, RoomId};

use crate::{
    components::{molecules::input_message::ReplyingTo, organisms::chat::utils::handle_command},
    services::matrix::matrix::send_message, pages::chat::chat::MessageItem,
};

use super::use_client::use_client;

#[allow(clippy::needless_return)]
pub fn use_send_message(cx: &ScopeState) -> &UseSendMessageState {
    let client = use_client(cx).get();
    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();

    let task_push_message = use_coroutine(cx, |mut rx: UnboundedReceiver<MessageItem>| {
        to_owned![client, replying_to];

        async move {
            while let Some(message_item) = rx.next().await {
                if message_item.msg.starts_with('!') {
                    handle_command(message_item, &client).await;
                } else {
                    let room_id = RoomId::parse(message_item.room_id).unwrap();
                    let event_id = if let Some(e) = message_item.reply_to {
                        Some(EventId::parse(e).unwrap())
                    } else {
                        None
                    };
                    send_message(&client, &room_id, message_item.msg, event_id).await
                }

                *replying_to.write() = None;
            }
        }
    });

    cx.use_hook(move || UseSendMessageState {
        inner: task_push_message.clone(),
    })
}

#[derive(Clone)]
pub struct UseSendMessageState {
    inner: Coroutine<MessageItem>,
}

impl UseSendMessageState {
    pub fn send(&self, message: MessageItem) {
        self.inner.send(message)
    }
}
