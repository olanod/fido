use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures_util::StreamExt;
use matrix_sdk::ruma::{EventId, RoomId};
use uuid::Uuid;

use crate::{
    components::molecules::input_message::ReplyingTo,
    hooks::{factory::message_factory::MessageFactory, use_send_message::get_current_time},
    services::matrix::matrix::{
        send_attachment, upload_attachment, AttachmentStream, FileContent, TimelineMessageType,
        TimelineRelation, TimelineThread,
    },
};

use super::{
    factory::message_factory::{
        use_custom_thread_message_factory, use_reply_message_factory, use_text_message_factory,
    },
    use_attach::use_attach,
    use_client::use_client,
    use_init_app::MessageDispatchId,
    use_messages::use_messages,
    use_notification::use_notification,
    use_room::use_room,
};

pub enum SendAttachStatus {
    Loading(i8),
    Sending,
    Sent,
    Error,
    None,
}

#[allow(clippy::needless_return)]
pub fn use_send_attach(cx: &ScopeState) -> &UseSendMessageState {
    let i18 = use_i18(cx);
    let client = use_client(cx).get();
    let room = use_room(cx).get();
    let notification = use_notification(cx);
    let messages = use_messages(cx);
    let attach = use_attach(cx);
    let text_message_factory = use_text_message_factory(cx);
    let reply_message_factory = use_reply_message_factory(cx);
    let custom_thread_message_factory = use_custom_thread_message_factory(cx);

    let key_common_error_thread_id = translate!(i18, "chat.common.error.thread_id");
    let key_common_error_event_id = translate!(i18, "chat.common.error.event_id");
    let key_common_error_room_id = translate!(i18, "chat.common.error.room_id");

    let key_attach_error_upload_file = translate!(i18, "chat.attach.error.upload_file");
    let key_attach_error_send_message = translate!(i18, "chat.attach.error.send_message");

    let send_attach_status =
        use_shared_state::<SendAttachStatus>(cx).expect("Unable to use SendAttachStatus");
    let message_dispatch_id =
        use_shared_state::<MessageDispatchId>(cx).expect("Unable to use MessageDispatchId");
    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).expect("Unable to use ReplyingTo");
    let threading_to =
        use_shared_state::<Option<TimelineThread>>(cx).expect("Cannot found thread_to");

    let task_push_attach = use_coroutine(cx, |mut rx: UnboundedReceiver<AttachmentStream>| {
        to_owned![
            client,
            replying_to,
            threading_to,
            notification,
            messages,
            message_dispatch_id,
            send_attach_status,
            attach
        ];

        async move {
            while let Some(message_item) = rx.next().await {
                *send_attach_status.write() = SendAttachStatus::Loading(0);

                let room_id = match RoomId::parse(&room.id) {
                    Ok(id) => id,
                    Err(_) => {
                        notification.handle_error("{key_common_error_room_id}");
                        return;
                    }
                };

                let reply_to = replying_to.read().clone();
                let thread_to = threading_to.read().clone();

                let reply_event_id = match reply_to {
                    Some(e) => {
                        let event_id = match EventId::parse(e.event_id) {
                            Ok(id) => id,
                            Err(_) => {
                                notification.handle_error("key_common_error_event_id");
                                return;
                            }
                        };
                        Some(event_id)
                    }
                    None => None,
                };

                let thread_event_id = &thread_to.clone().and_then(|e| {
                    if message_item.send_to_thread {
                        EventId::parse(e.event_id.clone())
                            .map_err(|_| {
                                notification.handle_error("key_common_error_thread_id");
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
                                notification.handle_error("key_common_error_thread_id");
                            })
                            .ok()
                    } else {
                        None
                    }
                });

                let mut back_messages = messages.get().clone();
                let timestamp = get_current_time();

                let uuid = Uuid::new_v4();

                message_dispatch_id
                    .write()
                    .value
                    .insert(uuid.to_string(), None);

                *send_attach_status.write() = SendAttachStatus::Loading(25);

                if let Some(file) = attach.get() {
                    let message_to_push = if let Some(r) = replying_to.clone().read().clone() {
                        reply_message_factory.create_message(
                            &TimelineMessageType::Image(FileContent {
                                size: Some(file.size.clone()),
                                body: file.name.clone(),
                                source: Some(crate::services::matrix::matrix::ImageType::Media(
                                    file.data.clone(),
                                )),
                            }),
                            &uuid.to_string(),
                            &timestamp,
                            &r,
                        )
                    } else if let Some(mut thread) = thread_to.to_owned() {
                        let custom_thread = custom_thread_message_factory.create_message(
                            &TimelineMessageType::Image(FileContent {
                                size: Some(file.size.clone()),
                                body: file.name.clone(),
                                source: Some(crate::services::matrix::matrix::ImageType::Media(
                                    file.data.clone(),
                                )),
                            }),
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
                            &TimelineMessageType::Image(FileContent {
                                size: Some(file.size.clone()),
                                body: file.name.clone(),
                                source: Some(crate::services::matrix::matrix::ImageType::Media(
                                    file.data.clone(),
                                )),
                            }),
                            &uuid.to_string(),
                            &timestamp,
                            &String::new(),
                        )
                    };
                    messages.push(message_to_push);
                }

                *replying_to.write() = None;
                attach.reset();

                let response = upload_attachment(&client, &message_item.attachment).await;

                let uri = match response {
                    Ok(r) => r.content_uri,
                    Err(_) => {
                        notification.handle_error("{key_attach_error_upload_file}");
                        return;
                    }
                };

                let response = send_attachment(
                    &client,
                    &room_id,
                    &uri,
                    &message_item.attachment,
                    reply_event_id,
                    thread_event_id.clone(),
                    latest_event_id,
                )
                .await;

                match response {
                    Ok(r) => {
                        message_dispatch_id
                            .write()
                            .value
                            .insert(uuid.to_string(), Some(r.event_id.to_string()));
                    }
                    Err(_) => {
                        notification.handle_error("{key_attach_error_send_message}");
                        return;
                    }
                };
            }
        }
    });

    cx.use_hook(move || UseSendMessageState {
        inner: task_push_attach.clone(),
    })
}

#[derive(Clone)]
pub struct UseSendMessageState {
    inner: Coroutine<AttachmentStream>,
}

impl UseSendMessageState {
    pub fn send(&self, message: AttachmentStream) {
        self.inner.send(message)
    }
}
