use std::ops::Deref;

use dioxus::{html::input_data::keyboard_types, prelude::*};
use log::info;
use matrix_sdk::ruma::{RoomId, UInt};

use crate::{
    components::{atoms::{message::Message, Attach, MessageInput, MessageView, Button, header_main::{HeaderEvent, HeaderCallOptions}, input::InputType, hover_menu::{MenuEvent, MenuOption}, Icon, Dollar,
    }, molecules::{AttachPreview, PaymentPreview}},
    services::matrix::matrix::{TimelineMessageType, EventOrigin, Attachment}, hooks::{use_attach::{use_attach, AttachFile}, use_client::use_client, use_room::use_room},
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

#[derive(Debug)]
pub struct Payment {
    value: i32,
}

pub fn InputMessage<'a>(cx: Scope<'a, InputMessageProps<'a>>) -> Element<'a> {
    let attach = use_attach(cx);
    let client = use_client(cx);
    let room = use_room(cx);
    let message_field = use_state(cx, String::new);
    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();
    use_shared_state_provider::<Option<Payment>>(cx, || None);
    let payment = use_shared_state::<Option<Payment>>(cx).unwrap();
    
    let wrapper_style = r#"
        flex-direction: column;
    "#;

    let container_style = r#"
        display: flex;
        gap: 0.75rem;
    "#;

    let on_handle_send_attach = move |_| {
        attach.reset();
    };

    let on_handle_attach = move |event: Event<FormData>| {
        cx.spawn({
            to_owned![attach];

            async move {
                let files = &event.files;
                
                if let Some(f) = &files {
                    let fs = f.files();
                    let file = f.read_file(fs.get(0).unwrap()).await;

                    if let Some(content) = file {
        
                        let x = infer::get(content.deref());

                        info!("type: {:?}", x.unwrap().mime_type());
                        
                        let content_type: mime::Mime = x.unwrap().mime_type().parse().unwrap();
                        

                        let blob = match content_type.type_() {
                            mime::IMAGE => {
                                gloo::file::Blob::new(content.deref())
                            },
                            mime::VIDEO => {
                                gloo::file::Blob::new_with_options(content.deref(), Some(x.unwrap().mime_type()))
                            },
                            _ => {
                                gloo::file::Blob::new(content.deref())
                            }
                        };


                        let size = blob.size().clone();
                        info!("mime: {:?}", content_type);
                        let object_url = gloo::file::ObjectUrl::from(blob);
                        
                        attach.set(Some(AttachFile { 
                            name: fs.get(0).unwrap().to_string(), 
                            preview_url: object_url, 
                            data: content.clone(), 
                            content_type,
                            size
                        })) ;
                    }
                }
            }
        });
    };

    let button_style = r#"
        cursor: pointer;
        background: var(--background-button);
        border: none;
        border-radius: 100%;
        max-width: 2.625rem;
        width: 100%;
        height: 2.625rem;
    "#;

    cx.render(rsx! {
      div {
        id: "input_field",
        style: "{wrapper_style}",
        class: "input__message",

        if let Some(replying) = replying_to.read().deref() {
            rsx!(
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
                AttachPreview {}
            )
        } 

        if let Some(_) = *payment.read() {
            rsx!(
                PaymentPreview {}
            )
        } 
        
        div {
            style: "{container_style}", 
            if message_field.get().len() == 0 {
                rsx!(
                    if let Some(_) = &cx.props.on_attach {
                        rsx!(
                            Attach {
                                on_click: on_handle_attach
                            }
                        )
                    } else {
                        rsx!(div {})
                    }
        
                    button {
                        style: "{button_style}",
                        onclick: move |_| {
                            *payment.write() = Some(Payment {value: 3})
                        },
                        Icon {
                            stroke: "var(--icon-white)",
                            icon: Dollar
                        }
                    }
                )
            }
            
            if let Some(file) = attach.get().clone() {
                rsx!(
                    Button {
                        text: "Enviar",
                        on_click: move |event| {
                            if let Some(l) = &cx.props.on_attach {
                                let attachment = Attachment {
                                    body: file.name.clone(),
                                    data: file.data.clone(),
                                    content_type: file.content_type.clone()
                                };

                                l.call(attachment);

                                on_handle_send_attach(event);
                            }
                        }
                    }
                )
            } else {
                rsx!(
                    MessageInput {
                        message: "{message_field}",
                        placeholder: cx.props.placeholder,
                        itype: cx.props.message_type.clone(),
                        error: None,
                        on_input: move |event: FormEvent| {
                            message_field.set(event.value.clone());
                            if message_field.get().starts_with("@") {
                                cx.spawn({
                                    to_owned!(room, client);

                                    async move {
                                        let current_room = room.get();
                                                                        
                                        let room = client.get().get_room(&RoomId::parse(current_room.id).unwrap());

                                        if let Some(r) = room {
                                            let members = r.members().await;

                                            if let Ok(m) = members {
                                                
                                                
                                            }
                                        }
                                    }
                                })
                            }
                        },
                        on_keypress: move |event: KeyboardEvent| {
                            if event.code() == keyboard_types::Code::Enter && message_field.get().len() > 0 {
                                cx.props.on_submit.call(FormMessageEvent { value: message_field.get().clone() });
                                message_field.set(String::new());
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
