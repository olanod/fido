use chrono::{DateTime, Local};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;
use matrix_sdk::ruma::{
    events::room::message::{MessageType, TextMessageEventContent},
    EventId, RoomId,
};
use ruma::{OwnedEventId, OwnedRoomId};
use std::time::{Duration, UNIX_EPOCH};
use uuid::Uuid;

use crate::{
    components::organisms::chat::utils::handle_command,
    hooks::factory::message_factory::MessageFactory,
    pages::chat::chat::MessageItem,
    services::matrix::matrix::{send_message, TimelineMessageType, TimelineRelation},
};

use super::{
    factory::message_factory::use_message_factory,
    use_client::{use_client, UseClientState},
    use_init_app::MessageDispatchId,
    use_messages::use_messages,
    use_notification::use_notification,
    use_reply::use_reply,
    use_session::use_session,
    use_thread::{use_thread, UseThreadState},
};

pub enum SendMessageError {
    InvalidRoom,
    InvalidReplyEventId,
    InvalidThreadEventId,
    DispatchMessage,
    RoomNotFound,
    InvalidFile,
}

#[derive(Clone)]
pub enum MessageStatus {
    Sent(String),
    Error,
    None,
}

#[allow(clippy::needless_return)]
pub fn use_send_message(cx: &ScopeState) -> &UseSendMessageState {
    let i18 = use_i18(cx);
    let client = use_client(cx);
    let notification = use_notification(cx);
    let messages = use_messages(cx);
    let session = use_session(cx);
    let replying_to = use_reply(cx);
    let threading_to = use_thread(cx);
    let message_factory = use_message_factory(cx);

    let key_common_error_thread_id = translate!(i18, "chat.common.error.thread_id");
    let key_common_error_event_id = translate!(i18, "chat.common.error.event_id");
    let key_common_error_room_id = translate!(i18, "chat.common.error.room_id");
    let key_common_error_user_id = translate!(i18, "chat.common.error.user_id");
    let key_common_error_file_type = translate!(i18, "chat.common.error.file_type");

    let key_message_error_send_message = translate!(i18, "chat.message.error.send_message");

    let key_commands_join_errors_room_not_found =
        translate!(i18, "chat.commands.join.errors.room_not_found");
    let key_commands_join_errors_action_not_found =
        translate!(i18, "chat.commands.join.errors.action_not_found");
    let key_commands_join_errors_invalid_room =
        translate!(i18, "chat.commands.join.errors.invalid_room");
    let key_commands_join_errors_request_failed =
        translate!(i18, "chat.commands.join.errors.request_failed");

    let message_dispatch_id =
        use_shared_state::<MessageDispatchId>(cx).expect("Unable to use MessageDispatchId");

    let value = use_ref::<MessageStatus>(cx, || MessageStatus::None);

    let task_push = use_coroutine(cx, |mut rx: UnboundedReceiver<MessageItem>| {
        to_owned![
            client,
            replying_to,
            threading_to,
            notification,
            messages,
            session,
            message_dispatch_id,
            message_factory
        ];

        async move {
            while let Some(message_item) = rx.next().await {
                if message_item.msg.starts_with('!') {
                    let Err(error) = handle_command(&message_item, &client.get()).await else {
                        return;
                    };

                    let message = match error {
                        handle_command::CommandError::RoomIdNotFound => {
                            &key_commands_join_errors_room_not_found
                        }
                        handle_command::CommandError::ActionNotFound => {
                            &key_commands_join_errors_action_not_found
                        }
                        handle_command::CommandError::InvalidRoomId => {
                            &key_commands_join_errors_invalid_room
                        }
                        handle_command::CommandError::RequestFailed => {
                            &key_commands_join_errors_request_failed
                        }
                    };

                    notification.handle_error(message)
                } else {
                    let mut back_messages = messages.get();
                    let uuid = Uuid::new_v4().to_string();

                    let session_data = match session.get() {
                        Some(user) => user,
                        None => {
                            notification.handle_error(&key_common_error_user_id);
                            return;
                        }
                    };

                    let timestamp = get_current_time();
                    message_dispatch_id
                        .write()
                        .value
                        .insert(uuid.clone().into(), None);

                    // build message relation
                    let message_to_push = if let Some(r) = replying_to.get().clone() {
                        message_factory.reply(r).create_message(
                            &TimelineMessageType::Text(message_item.msg.clone().into()),
                            &uuid,
                            &timestamp,
                            &session_data,
                        )
                    } else if let Some(thread) = threading_to.get().to_owned() {
                        message_factory.thread(thread).create_message(
                            &TimelineMessageType::Text(message_item.msg.clone().into()),
                            &uuid,
                            &timestamp,
                            &session_data,
                        )
                    } else {
                        message_factory.text().create_message(
                            &TimelineMessageType::Text(message_item.msg.clone().into()),
                            &uuid,
                            &timestamp,
                            &session_data,
                        )
                    };

                    replying_to.set(None);

                    if let TimelineRelation::None(_) | TimelineRelation::Reply(_) = message_to_push
                    {
                        messages.push(message_to_push);
                    } else if let TimelineRelation::CustomThread(ref t) = message_to_push {
                        let position = back_messages.iter().position(|m| {
                            let TimelineRelation::CustomThread(n) = m else {
                                return false;
                            };

                            n.event_id.eq(&t.event_id)
                        });

                        if let Some(p) = position {
                            threading_to.set(Some(t.clone()));
                            back_messages[p] = message_to_push
                        }
                    };

                    let event_id = process(
                        &client,
                        &message_item.room_id,
                        message_item.reply_to.as_ref(),
                        &threading_to,
                        &message_item.msg,
                        message_item.send_to_thread,
                    )
                    .await
                    .map_err(|e| {
                        let message = match e {
                            SendMessageError::RoomNotFound | SendMessageError::InvalidRoom => {
                                &key_common_error_room_id
                            }
                            SendMessageError::InvalidReplyEventId => &key_common_error_event_id,
                            SendMessageError::InvalidThreadEventId => &key_common_error_thread_id,
                            SendMessageError::DispatchMessage => &key_message_error_send_message,
                            SendMessageError::InvalidFile => &key_common_error_file_type,
                        };

                        notification.handle_error(message);
                        return;
                    })
                    .ok();

                    message_dispatch_id.write().value.insert(uuid, event_id);
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

pub async fn process(
    client: &UseClientState,
    room_id: &str,
    reply_to: Option<&String>,
    thread_to: &UseThreadState,
    content: &str,
    send_to_thread: bool,
) -> Result<String, SendMessageError> {
    let thread_to = thread_to.get();
    let room_id = RoomId::parse(room_id).map_err(|_| SendMessageError::InvalidRoom)?;

    let reply_event_id = reply_to
        .map(|e| EventId::parse(&e))
        .transpose()
        .map_err(|_| SendMessageError::InvalidReplyEventId)?;

    let mut thread_event_id = None;
    let mut latest_event_id = None;

    if send_to_thread {
        thread_event_id = thread_to
            .as_ref()
            .map(|e| EventId::parse(&e.event_id))
            .transpose()
            .map_err(|_| SendMessageError::InvalidThreadEventId)?;

        latest_event_id = thread_to
            .as_ref()
            .map(|e| EventId::parse(&e.latest_event))
            .transpose()
            .map_err(|_| SendMessageError::InvalidThreadEventId)?;
    }

    let event_id = dispatch_message(
        &client,
        room_id,
        reply_event_id,
        thread_event_id,
        latest_event_id,
        &content,
    )
    .await?;

    Ok(event_id)
}

async fn dispatch_message(
    client: &UseClientState,
    room_id: OwnedRoomId,
    reply_event_id: Option<OwnedEventId>,
    thread_event_id: Option<OwnedEventId>,
    latest_event_id: Option<OwnedEventId>,
    content: &str,
) -> Result<String, SendMessageError> {
    let response = send_message(
        &client.get(),
        &room_id,
        MessageType::Text(TextMessageEventContent::plain(content)),
        reply_event_id,
        thread_event_id,
        latest_event_id,
    )
    .await;

    let r = response.map_err(|_| SendMessageError::DispatchMessage)?;

    Ok(r.event_id.to_string())
}
