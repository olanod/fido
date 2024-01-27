use chrono::{DateTime, Local};
use dioxus::prelude::*;
use futures_util::StreamExt;
use log::info;
use matrix_sdk::ruma::{
    events::room::message::{MessageType, TextMessageEventContent},
    EventId, RoomId,
};
use std::time::{Duration, UNIX_EPOCH};
use uuid::Uuid;

use crate::{
    components::{molecules::input_message::ReplyingTo, organisms::chat::utils::handle_command},
    hooks::factory::message_factory::MessageFactory,
    pages::chat::chat::MessageItem,
    services::matrix::matrix::{
        send_message, TimelineMessageType, TimelineRelation, TimelineThread,
    },
};

use super::{
    factory::message_factory::{
        use_custom_thread_message_factory, use_reply_message_factory, use_text_message_factory,
    },
    use_client::use_client,
    use_init_app::MessageDispatchId,
    use_messages::use_messages,
    use_notification::use_notification,
    use_session::use_session,
};

#[derive(Clone)]
pub enum MessageStatus {
    Sent(String),
    Error,
    None,
}

#[allow(clippy::needless_return)]
pub fn use_send_message(cx: &ScopeState) -> &UseSendMessageState {
    let client = use_client(cx).get();
    let notification = use_notification(cx);
    let messages = use_messages(cx);
    let session = use_session(cx);
    let text_message_factory = use_text_message_factory(cx);
    let reply_message_factory = use_reply_message_factory(cx);
    let custom_thread_message_factory = use_custom_thread_message_factory(cx);

    let message_item = use_state::<Option<MessageItem>>(cx, || None);
    let value = use_ref::<MessageStatus>(cx, || MessageStatus::None);

    let message_dispatch_id =
        use_shared_state::<MessageDispatchId>(cx).expect("Unable to use MessageDispatchId");
    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).expect("Unable to use ReplyingTo");
    let threading_to =
        use_shared_state::<Option<TimelineThread>>(cx).expect("Cannot found thread_to");

    let task_push = use_coroutine(cx, |mut rx: UnboundedReceiver<MessageItem>| {
        to_owned![
            client,
            replying_to,
            threading_to,
            notification,
            message_item,
            value,
            messages,
            session,
            message_dispatch_id
        ];

        async move {
            while let Some(message_item) = rx.next().await {
                if message_item.msg.starts_with('!') {
                    handle_command(&message_item, &client).await;
                } else {
                    let room_id = match RoomId::parse(message_item.room_id.clone()) {
                        Ok(id) => id,
                        Err(_) => {
                            notification.handle_error("Error inesperado: (Id de sala)");
                            return;
                        }
                    };

                    let thread_to = threading_to.read().clone();

                    let reply_event_id = message_item.reply_to.clone().and_then(|e| {
                        EventId::parse(e)
                            .map_err(|_| {
                                notification.handle_error("Error inesperado: (Id de evento reply)")
                            })
                            .ok()
                    });

                    let thread_event_id = &thread_to.clone().and_then(|e| {
                        if message_item.send_to_thread {
                            EventId::parse(e.event_id.clone())
                                .map_err(|_| {
                                    notification
                                        .handle_error("Error inesperado: (Id de evento thread)");
                                })
                                .ok()
                        } else {
                            None
                        }
                    });

                    let latest_event_id = thread_to.clone().and_then(|e| {
                        if message_item.send_to_thread {
                            EventId::parse(e.latest_event)
                                .map_err(|_| {
                                    notification
                                        .handle_error("Error inesperado: (Id de evento latest)");
                                })
                                .ok()
                        } else {
                            None
                        }
                    });

                    let mut back_messages = messages.get().messages.clone();
                    let timestamp = get_current_time();

                    let uuid = Uuid::new_v4();

                    message_dispatch_id
                        .write()
                        .value
                        .insert(uuid.to_string(), None);

                    let message_to_push = if let Some(r) = replying_to.clone().read().clone() {
                        reply_message_factory.create_message(
                            &TimelineMessageType::Text(message_item.msg.clone()),
                            &uuid.to_string(),
                            &timestamp,
                            &r,
                        )
                    } else if let Some(mut thread) = thread_to.to_owned() {
                        let custom_thread = custom_thread_message_factory.create_message(
                            &TimelineMessageType::Text(message_item.msg.clone()),
                            &uuid.to_string(),
                            &timestamp,
                            &thread,
                        );

                        let t = if let TimelineRelation::CustomThread(ref t) = custom_thread {
                            t
                        } else {
                            return;
                        };

                        let position = back_messages.iter().position(|m| {
                            if let TimelineRelation::CustomThread(n) = m {
                                n.event_id.eq(&t.event_id)
                            } else {
                                false
                            }
                        });

                        *threading_to.write() = Some(t.clone());

                        if let Some(p) = position {
                            back_messages[p] = custom_thread.clone()
                        }

                        custom_thread
                    } else {
                        text_message_factory.create_message(
                            &TimelineMessageType::Text(message_item.msg.clone()),
                            &uuid.to_string(),
                            &timestamp,
                            &String::new(),
                        )
                    };

                    *replying_to.write() = None;

                    match message_to_push {
                        TimelineRelation::None(_) | TimelineRelation::Reply(_) => {
                            messages.push(message_to_push);
                        }
                        TimelineRelation::Thread(_) | TimelineRelation::CustomThread(_) => {}
                    }

                    let response = send_message(
                        &client,
                        &room_id,
                        MessageType::Text(TextMessageEventContent::plain(message_item.msg.clone())),
                        reply_event_id,
                        thread_event_id.clone(),
                        latest_event_id,
                    )
                    .await;

                    match response {
                        Ok(r) => {
                            MessageStatus::Sent(r.event_id.to_string());
                            message_dispatch_id
                                .write()
                                .value
                                .insert(uuid.to_string(), Some(r.event_id.to_string()));
                        }
                        Err(_) => {
                            notification.handle_error("No se ha podido enviar el mensaje");
                        }
                    };
                }
            }
        }
    });

    cx.use_hook(move || UseSendMessageState {
        inner: task_push.clone(),
        value: value.clone(),
    })
}

#[derive(Clone)]
pub struct UseSendMessageState {
    inner: Coroutine<MessageItem>,
    value: UseRef<MessageStatus>,
}

impl UseSendMessageState {
    pub fn send(&self, message: MessageItem) {
        self.inner.send(message)
    }

    pub fn get_value(&self) -> MessageStatus {
        self.value.read().clone()
    }
}

pub fn get_current_time() -> String {
    let time_now = matrix_sdk::instant::now();
    let time_duration = UNIX_EPOCH + Duration::from_millis((time_now).round() as u64);

    let timestamp = {
        let datetime = DateTime::<Local>::from(time_duration);
        datetime.format("%H:%M").to_string()
    };

    timestamp
}
