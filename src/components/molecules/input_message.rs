use std::ops::Deref;

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;

use crate::{
    components::{
        atoms::{
            header_main::{HeaderCallOptions, HeaderEvent},
            hover_menu::{MenuEvent, MenuOption},
            message::MessageView,
            Attach, Button, Close, Icon, Message, TextareaInput,
        },
        molecules::AttachPreview,
    },
    hooks::{
        use_attach::{use_attach, AttachError, AttachFile},
        use_notification::use_notification,
        use_reply::use_reply,
    },
    services::matrix::matrix::{Attachment, EventOrigin, TimelineMessageType},
};

#[derive(Debug, Clone)]
pub struct FormMessageEvent {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ReplyingTo {
    pub event_id: String,
    pub content: TimelineMessageType,
    pub display_name: String,
    pub avatar_uri: Option<String>,
    pub origin: EventOrigin,
}

#[derive(PartialEq, Props, Clone)]
pub struct InputMessageProps {
    placeholder: String,
    on_submit: EventHandler<FormMessageEvent>,
    on_event: EventHandler<HeaderEvent>,
    on_attach: Option<EventHandler<Attachment>>,
}

pub fn InputMessage(props: InputMessageProps) -> Element {
    let i18 = use_i18();
    let mut attach = use_attach();
    let mut notification = use_notification();
    let mut replying_to = use_reply();

    let mut message_field = use_signal(String::new);
    let mut wrapper_style = use_signal(|| {
        r#"
            flex-direction: column;
        "#
    });

    let mut on_handle_send_attach = move || {
        wrapper_style.set(
            r#"
                flex-direction: column;
            "#,
        );
    };

    let on_handle_attach = move |event: Event<FormData>| {
        spawn({
            async move {
                let files = &event.files().ok_or(AttachError::NotFound)?;
                let fs = files.files();

                let existing_file = fs.get(0).ok_or(AttachError::NotFound)?;
                let content = files
                    .read_file(existing_file)
                    .await
                    .ok_or(AttachError::NotFound)?;
                let infered_type = infer::get(content.deref()).ok_or(AttachError::UncoverType)?;

                let content_type: Result<mime::Mime, _> = infered_type.mime_type().parse();
                let content_type = content_type.map_err(|_| AttachError::UnknownContent)?;

                let blob = match content_type.type_() {
                    mime::IMAGE => gloo::file::Blob::new(content.deref()),
                    mime::VIDEO => gloo::file::Blob::new_with_options(
                        content.deref(),
                        Some(infered_type.mime_type()),
                    ),
                    _ => gloo::file::Blob::new(content.deref()),
                };

                let size = blob.size().clone();
                let object_url = gloo::file::ObjectUrl::from(blob);

                attach.set(Some(AttachFile {
                    name: existing_file.to_string(),
                    preview_url: object_url,
                    data: content.clone(),
                    content_type,
                    size,
                }));

                wrapper_style.set(
                    r#"
                        flex-direction: column;
                        position: absolute;
                        height: calc(100vh - 70px);
                        background: var(--background);
                    "#,
                );

                Ok::<(), AttachError>(())
            }
            .unwrap_or_else(move |e: AttachError| {
                let message_error = match e {
                    AttachError::NotFound => translate!(i18, "chat.input_message.not_found"),
                    AttachError::UncoverType => translate!(i18, "chat.input_message.file_type"),
                    AttachError::UnknownContent => {
                        translate!(i18, "chat.input_message.unknown_content")
                    }
                };

                notification.handle_error(&message_error);
            })
        });
    };

    let mut on_handle_file_cta = move |file: AttachFile| {
        if let Some(l) = props.on_attach {
            let attachment = Attachment {
                body: file.name.clone(),
                data: file.data.clone(),
                content_type: file.content_type.clone(),
            };

            l.call(attachment);
            on_handle_send_attach();
        }
    };

    let mut on_key_press = move |event: KeyboardEvent| {
        let modifiers = event.modifiers();

        match modifiers {
            keyboard_types::Modifiers::SHIFT => {}
            _ => {
                if event.code() == keyboard_types::Code::Enter {
                    if !message_field().trim().is_empty() {
                        props.on_submit.call(FormMessageEvent {
                            value: message_field(),
                        });
                    }
                    message_field.set(String::from(""));
                }
            }
        }
    };

    rsx! {
      div {
        id: "input_field",
        style: "{wrapper_style.read()}",
        class: "input__message",

        if let Some(replying) = replying_to.get() {
            div {
                class: "input__message__replying",
                span {
                    class: "input__message__title",
                    {translate!(i18, "chat.input_message.subtitle")}
                }
                button {
                    class: "input__message__close",
                    onclick: move |_| {props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
                    Icon {
                        stroke: "var(--icon-subdued)",
                        icon: Close
                    }
                }
            }

            MessageView {
                message: Message {
                    id: 1,
                    display_name: replying.display_name.clone(),
                    event_id: String::from(""),
                    avatar_uri: replying.avatar_uri.clone(),
                    content: replying.content.clone(),
                    reply: None,
                    origin: replying.origin.clone(),
                    time: String::from(""),
                    thread: None
                },
                is_replying: true,
                on_event: move |event: MenuEvent| {
                    if let MenuOption::Close = event.option {
                        props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })
                    }
                }
            }
        }

        if let Some(_) = attach.get() {
            AttachPreview {
                on_event: move |_| {
                    on_handle_send_attach();
                    attach.reset();
                }
            }
        }

        div {
            class: "input__message__container",
            if let Some(_) = &props.on_attach {
                Attach {
                    on_click: on_handle_attach
                }
            }

            if let Some(file) = attach.get().clone() {
                Button {
                    text: translate!(i18, "chat.input_message.cta"),
                    status: None,
                    on_click: move |_| {
                        on_handle_file_cta(file.clone());
                    }
                }
            } else {
                TextareaInput {
                    value: "{message_field}",
                    placeholder: props.placeholder,
                    on_input: move |event: FormEvent| {
                        message_field.set(event.value());
                    },
                    on_keypress: on_key_press,
                    on_click: move |_| {
                        props.on_submit.call(FormMessageEvent { value: message_field() });
                        message_field.set(String::new());
                    }
                }
            }
        }
      }
    }
}
