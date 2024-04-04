use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;
use matrix_sdk::ruma::{EventId, RoomId};
use ruma::{OwnedEventId, OwnedRoomId};
use uuid::Uuid;

use crate::{
    hooks::{factory::message_factory::MessageFactory, use_send_message::get_current_time},
    services::matrix::matrix::{
        send_attachment, upload_attachment, Attachment, AttachmentStream, FileContent,
        TimelineMessageType, TimelineRelation,
    },
};

use super::{
    factory::message_factory::use_message_factory,
    use_attach::{use_attach, UseAttachState},
    use_client::{use_client, UseClientState},
    use_init_app::MessageDispatchId,
    use_messages::use_messages,
    use_notification::use_notification,
    use_reply::{use_reply, UseReplyState},
    use_room::use_room,
    use_send_message::SendMessageError,
    use_session::use_session,
    use_thread::{use_thread, UseThreadState},
};

pub enum SendAttachStatus {
    Loading(i8),
    Sending,
    Sent,
    Error,
    None,
}

pub fn use_send_attach() -> UseSendMessageState {
    let i18 = use_i18();
    let client = use_client();
    let room = use_room().get();
    let mut notification = use_notification();
    let mut messages = use_messages();
    let mut attach = use_attach();
    let session = use_session();
    let message_factory = use_message_factory();

    let mut send_attach_status = consume_context::<Signal<SendAttachStatus>>();
    let mut message_dispatch_id = consume_context::<Signal<MessageDispatchId>>();
    let mut replying_to = use_reply();
    let mut threading_to = use_thread();

    let task_push_attach = use_coroutine(|mut rx: UnboundedReceiver<AttachmentStream>| {
        async move {
            while let Some(message_item) = rx.next().await {
                let mut back_messages = messages.get().clone();
                let timestamp = get_current_time();
                let thread_to = threading_to.get();
                let uuid = Uuid::new_v4().to_string();

                *send_attach_status.write() = SendAttachStatus::Loading(25);

                if let Some(file) = attach.get() {
                    // build message relation
                    let content_type = file.content_type.type_();

                    match content_type {
                        mime::IMAGE => {
                            crate::services::matrix::matrix::ImageType::Media(file.data.clone())
                        }
                        mime::VIDEO => {
                            crate::services::matrix::matrix::ImageType::Media(file.data.clone())
                        }
                        mime::APPLICATION => {
                            crate::services::matrix::matrix::ImageType::Media(file.data.clone())
                        }
                        _ => {
                            notification
                                .handle_error(&translate!(i18, "chat.common.error.file_type"));
                            return;
                        }
                    };

                    let content = FileContent {
                        size: Some(file.size),
                        body: file.name,
                        source: Some(crate::services::matrix::matrix::ImageType::Media(file.data)),
                    };

                    let attach_type = match content_type {
                        mime::IMAGE => TimelineMessageType::Image(content),
                        mime::VIDEO => TimelineMessageType::Video(content),
                        mime::APPLICATION => TimelineMessageType::File(content),
                        _ => {
                            notification
                                .handle_error(&translate!(i18, "chat.common.error.file_type"));
                            return;
                        }
                    };
                    let message_to_push = if let Some(r) = replying_to.clone().get().clone() {
                        message_factory.reply(r).create_message(
                            &attach_type,
                            &uuid.to_string(),
                            &timestamp,
                            &session.get().unwrap(),
                        )
                    } else if let Some(thread) = thread_to.to_owned() {
                        message_factory.thread(thread).create_message(
                            &attach_type,
                            &uuid.to_string(),
                            &timestamp,
                            &session.get().unwrap(),
                        )
                    } else {
                        message_factory.text().create_message(
                            &attach_type,
                            &uuid.to_string(),
                            &timestamp,
                            &session.get().unwrap(),
                        )
                    };

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
                }

                let event_id = process(
                    &client,
                    &room.id,
                    &mut replying_to,
                    &mut threading_to,
                    &message_item.attachment,
                    message_item.send_to_thread,
                    &mut attach,
                )
                .await
                .map_err(|e| {
                    let message = match e {
                        SendMessageError::RoomNotFound | SendMessageError::InvalidRoom => {
                            translate!(i18, "chat.common.error.room_id")
                        }
                        SendMessageError::InvalidReplyEventId => {
                            translate!(i18, "chat.common.error.event_id")
                        }
                        SendMessageError::InvalidThreadEventId => {
                            translate!(i18, "chat.common.error.thread_id")
                        }
                        SendMessageError::DispatchMessage => {
                            translate!(i18, "chat.attach.error.send_message")
                        }
                        SendMessageError::InvalidFile => {
                            translate!(i18, "chat.common.error.file_type")
                        }
                    };

                    notification.handle_error(&message);
                    return;
                })
                .ok();

                message_dispatch_id.write().value.insert(uuid, event_id);
            }
        }
    });

    use_hook(move || UseSendMessageState {
        inner: task_push_attach.clone(),
    })
}

#[derive(Clone, Copy)]
pub struct UseSendMessageState {
    inner: Coroutine<AttachmentStream>,
}

impl UseSendMessageState {
    pub fn send(&self, message: AttachmentStream) {
        self.inner.send(message)
    }
}

pub async fn process(
    client: &UseClientState,
    room_id: &str,
    reply_to: &mut UseReplyState,
    thread_to: &mut UseThreadState,
    content: &Attachment,
    send_to_thread: bool,
    attach: &mut UseAttachState,
) -> Result<String, SendMessageError> {
    let thread_to = thread_to.get();
    let room_id = RoomId::parse(room_id).map_err(|_| SendMessageError::InvalidRoom)?;

    let reply_event_id = reply_to
        .get()
        .map(|e| EventId::parse(&e.event_id))
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

    reply_to.set(None);
    attach.reset();

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
    content: &Attachment,
) -> Result<String, SendMessageError> {
    let client = client.get();
    let response = upload_attachment(&client, &content)
        .await
        .map_err(|_| SendMessageError::DispatchMessage)?;

    let response = send_attachment(
        &client,
        &room_id,
        &response.content_uri,
        &content,
        reply_event_id,
        thread_event_id.clone(),
        latest_event_id,
    )
    .await;

    let r = response.map_err(|_| SendMessageError::DispatchMessage)?;

    Ok(r.event_id.to_string())
}
