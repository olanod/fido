use dioxus::prelude::*;
use futures_util::StreamExt;
use matrix_sdk::ruma::{
    events::room::message::{MessageType, TextMessageEventContent},
    EventId, RoomId,
};

use crate::{
    components::{molecules::input_message::ReplyingTo, organisms::chat::utils::handle_command},
    pages::chat::chat::MessageItem,
    services::matrix::matrix::{send_message, TimelineThread},
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

                    let reply_event_id = match message_item.reply_to {
                        Some(e) => Some(EventId::parse(e).unwrap()),
                        None => None,
                    };

                    let thread_event_id = match &thread_to {
                        Some(e) => {
                            if message_item.send_to_thread {
                                Some(EventId::parse(e.event_id.clone()).unwrap())
                            } else {
                                None
                            }
                        }
                        None => None,
                    };

                    let latest_event_id = match thread_to {
                        Some(e) => {
                            if message_item.send_to_thread {
                                Some(EventId::parse(e.latest_event).unwrap())
                            } else {
                                None
                            }
                        }
                        None => None,
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
