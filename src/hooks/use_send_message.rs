use dioxus::prelude::*;
use futures_util::StreamExt;
use log::info;
use matrix_sdk::ruma::{
    events::room::message::{MessageType, TextMessageEventContent},
    EventId, RoomId,
};

use crate::{
    components::{molecules::input_message::ReplyingTo, organisms::chat::utils::handle_command},
    pages::chat::chat::MessageItem,
    services::matrix::matrix::{send_message, TimelineMessageThread, TimelineThread},
};

use super::use_client::use_client;

#[allow(clippy::needless_return)]
pub fn use_send_message(cx: &ScopeState) -> &UseSendMessageState {
    let client = use_client(cx).get();
    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();
    let threading_to =
        use_shared_state::<Option<TimelineThread>>(cx).expect("Cannot found thread_to");

    let task_push_message = use_coroutine(cx, |mut rx: UnboundedReceiver<MessageItem>| {
        to_owned![client, replying_to, threading_to];

        async move {
            while let Some(message_item) = rx.next().await {
                if message_item.msg.starts_with('!') {
                    handle_command(message_item, &client).await;
                } else {
                    let room_id = RoomId::parse(message_item.room_id).unwrap();
                    let thread_to = threading_to.read().clone();

                    let reply_event_id = if let Some(e) = message_item.reply_to {
                        Some(EventId::parse(e).unwrap())
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

                    send_message(
                        &client,
                        &room_id,
                        MessageType::Text(TextMessageEventContent::plain(message_item.msg)),
                        reply_event_id,
                        thread_event_id,
                        latest_event_id,
                    )
                    .await
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
