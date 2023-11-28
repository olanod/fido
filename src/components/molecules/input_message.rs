use std::ops::Deref;

use dioxus::{html::input_data::keyboard_types, prelude::*};
use log::info;
use matrix_sdk::ruma::RoomId;

use crate::{
    components::{atoms::{message::Message, Attach, MessageView, Button, header_main::{HeaderEvent, HeaderCallOptions}, input::InputType, hover_menu::{MenuEvent, MenuOption}, TextareaInput, Close, Icon, Avatar,
    }, molecules::AttachPreview},
    services::matrix::matrix::{TimelineMessageType, EventOrigin, Attachment, RoomMember, room_members}, hooks::{use_attach::{use_attach, AttachFile}, use_client::use_client, use_room::use_room},
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
    let attach = use_attach(cx);
    let client = use_client(cx);
    let room = use_room(cx);
    let message_field = use_state(cx, String::new);
    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();
    let wrapper_style = use_ref(cx, || r#"
        flex-direction: column;
    "#);

    let container_style = r#"
        display: flex;
        gap: 0.75rem;
        align-items: flex-end;
    "#;

    let on_handle_send_attach = move || {
        attach.reset();
        wrapper_style.set(r#"
            flex-direction: column;
        "#);
    };

    let on_handle_attach = move |event: Event<FormData>| {
        cx.spawn({
            to_owned![attach, wrapper_style];

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

                        wrapper_style.set(r#"
                            flex-direction: column;
                            position: absolute;
                            height: 100vh;
                            background: var(--background);
                        "#);
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
                    style: "
                      display: flex;
                      justify-content: space-between;
                      align-items: center;
                      padding: 8px 0 4px; 
                    ",
                    span {
                        style: "
                            color: var(--text-1);
                            font-size: var(--size-1);
                        ",
                        "Respondiendo a "
                    }
                    button {
                      style: "{close_style}",
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
                        on_handle_send_attach()
                    }
                }
            )
        } 

        div {
            style: "{container_style}", 
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
                        text: "Enviar",
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
