use std::ops::Deref;

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::{atoms::{Attach, Button, header_main::{HeaderEvent, HeaderCallOptions}, input::InputType, hover_menu::{MenuEvent, MenuOption}, TextareaInput, Close, Icon, message::MessageView, Message,
    }, molecules::AttachPreview},
    services::matrix::matrix::{TimelineMessageType, EventOrigin, Attachment}, hooks::{use_attach::{use_attach, AttachFile}, use_client::use_client, use_room::use_room, use_send_attach::SendAttachStatus},
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

#[derive(Props)]
pub struct InputMessageProps<'a> {
    message_type: InputType,
    placeholder: &'a str,
    on_submit: EventHandler<'a, FormMessageEvent>,
    on_event: EventHandler<'a, HeaderEvent>,
    on_attach: Option<EventHandler<'a, Attachment>>
}

pub fn InputMessage<'a>(cx: Scope<'a, InputMessageProps<'a>>) -> Element<'a> {
    let i18 = use_i18(cx);
    let attach = use_attach(cx);
    let client = use_client(cx);
    let room = use_room(cx);

    let key_input_message_unknown_content = translate!(i18, "chat.input_message.unknown_content");
let key_input_message_file_type = translate!(i18, "chat.input_message.file_type");
let key_input_message_not_found = translate!(i18, "chat.input_message.not_found");
let key_input_message_cta = translate!(i18, "chat.input_message.cta");



    let message_field = use_state(cx, String::new);

    let send_attach_status =
        use_shared_state::<SendAttachStatus>(cx).expect("Unable to use SendAttachStatus");
    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).expect("Unable to use ReplyingTo");
    let error = use_state(cx, || None);
    let wrapper_style = use_ref(cx, || r#"
        flex-direction: column;
    "#);

    let on_handle_send_attach = move || {
        wrapper_style.set(r#"
            flex-direction: column;
        "#);
    };

    let on_handle_attach = move |event: Event<FormData>| {
        cx.spawn({
            to_owned![attach, wrapper_style, error];

            async move {
                let files = &event.files;
                
                if let Some(f) = &files {
                    let fs = f.files();
                    let first_file = fs.get(0);
                    
                    match first_file {
                        Some(existing_file) => {
                            let file = f.read_file(existing_file).await;

                            if let Some(content) = file {
                                let infer_type = infer::get(content.deref());

                                match infer_type {
                                    Some(infered_type) => {
                                        let content_type: Result<mime::Mime, _> = infered_type.mime_type().parse();
                                        match content_type {
                                            Ok(content_type) => {

                                                let blob = match content_type.type_() {
                                                    mime::IMAGE => {
                                                        gloo::file::Blob::new(content.deref())
                                                    },
                                                    mime::VIDEO => {
                                                        gloo::file::Blob::new_with_options(content.deref(), Some(infered_type.mime_type()))
                                                    },
                                                    _ => {
                                                        gloo::file::Blob::new(content.deref())
                                                    }
                                                };

                                                let size = blob.size().clone();
                                                let object_url = gloo::file::ObjectUrl::from(blob);
                                                
                                                attach.set(Some(AttachFile { 
                                                    name: existing_file.to_string(), 
                                                    preview_url: object_url, 
                                                    data: content.clone(), 
                                                    content_type,
                                                    size
                                                })) ;

                                                wrapper_style.set(r#"
                                                    flex-direction: column;
                                                    position: absolute;
                                                    height: calc(100vh - 70px);
                                                    background: var(--background);
                                                "#);
                                            }
                                            _ => {
                                                error.set(Some("{key_input_message_unknown_content}"));
                                            }
                                        }
                                    }
                                    None => {
                                        error.set(Some("{key_input_message_file_type}"))
                                    }
                                }
                            }
                        }
                        None => {
                            error.set(Some("{key_input_message_not_found}"))
                        }
                    }
                }
            }
        });
    };

    cx.render(rsx! {
      div {
        id: "input_field",
        style: "{wrapper_style.read()}",
        class: "input__message",

        if let Some(replying) = replying_to.read().deref() {
            let close_style = r#"
                cursor: pointer;
                background: transparent;
                border: 1px solid transparent;
                display: flex;
            "#;
              
            rsx!(
                div {
                    class: "input__message__replying",
                    span {
                        class: "input__message__title",
                        translate!(i18, "chat.input_message.subtitle")
                    }
                    button {
                      class: "input__message__close",
                      onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
                      Icon {
                        stroke: "var(--icon-subdued)",
                        icon: Close
                      }
                    }
                  }
                  
                MessageView {
                    key: "1",
                    message: Message {
                        id: 1,
                        display_name: replying.display_name.clone(),
                        event_id: None,
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
                            cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })
                        }
                    }
                }
            )
        }

        if let Some(_) = attach.get() {
            rsx!(
                AttachPreview {
                    on_event: move |event| {
                        on_handle_send_attach();
                        attach.reset();
                    }
                }
            )
        } 

        div {
            class: "input__message__container",
            if let Some(_) = &cx.props.on_attach {
                rsx!(
                    Attach {
                        on_click: on_handle_attach
                    }
                )
            }
            
            if let Some(file) = attach.get().clone() {
                rsx!(
                    Button {
                        text: "{key_input_message_cta}",
                        on_click: move |event| {
                            if let Some(l) = &cx.props.on_attach {
                                let attachment = Attachment {
                                    body: file.name.clone(),
                                    data: file.data.clone(),
                                    content_type: file.content_type.clone()
                                };

                                l.call(attachment);

                                on_handle_send_attach();
                            }
                        }
                    }
                )
            } else {
                rsx!(
                    TextareaInput {
                        value: "{message_field}",
                        placeholder: cx.props.placeholder,
                        on_input: move |event: FormEvent| {
                            let value = event.value.clone();
                            message_field.set(event.value.clone());
                        },
                        on_keypress: move |event: KeyboardEvent| {
                            let modifiers = event.modifiers();

                            match modifiers {
                                keyboard_types::Modifiers::SHIFT => {}
                                _ => {
                                    if event.code() == keyboard_types::Code::Enter && message_field.get().len() > 0 {
                                        cx.props.on_submit.call(FormMessageEvent { value: message_field.get().clone() });
                                        message_field.set(String::from(""));
                                    }
                                }
                            }
                        },
                        on_click: move |_| {
                            cx.props.on_submit.call(FormMessageEvent { value: message_field.get().clone() });
                            message_field.set(String::new());
                        }
                    }
                )
            }
        }
      }
    })
}
