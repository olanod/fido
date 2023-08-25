use dioxus::{html::input_data::keyboard_types, prelude::*};

use crate::{
    components::atoms::{
        header::HeaderEvent, message::Message, Attach, MessageInput, MessageView,
    },
    services::matrix::matrix::{TimelineMessageType, EventOrigin},
};

#[derive(Debug, Clone)]
pub struct FormMessageEvent {
    pub value: String,
}

#[derive(Debug)]
pub struct ReplyingTo {
    pub event_id: String,
    pub content: TimelineMessageType,
    pub display_name: String,
    pub avatar_uri: Option<String>,
    pub origin: EventOrigin,
}

#[derive(Props)]
pub struct InputMessageProps<'a> {
    message_type: &'a str,
    placeholder: &'a str,
    #[props(!optional)]
    replying_to: &'a Option<ReplyingTo>,
    is_attachable: bool,
    on_submit: EventHandler<'a, FormMessageEvent>,
    on_event: EventHandler<'a, HeaderEvent>,
}

pub fn InputMessage<'a>(cx: Scope<'a, InputMessageProps<'a>>) -> Element<'a> {
    let message_field = use_state(cx, String::new);

    let wrapper_style = r#"
        flex-direction: column;
    "#;

    let container_style = r#"
        display: flex;
        gap: 0.75rem;
    "#;
    cx.render(rsx! {
      div {
        id: "input_field",
        style: "{wrapper_style}",
        class: "input__message",
        if let Some(x) = cx.props.replying_to {
            rsx!(
                MessageView {
                    key: "1",
                    message: Message {
                        id: 1,
                        display_name: x.display_name.clone(),
                        event_id: None,
                        avatar_uri: x.avatar_uri.clone(),
                        content: x.content.clone(),
                        reply: None,
                        origin: x.origin.clone(),
                        time: String::from(""),
                    },
                    is_replying: true,
                    on_event: move |event| {cx.props.on_event.call(event)}
                }
            )
        }
        div {
            style: "{container_style}", 
            if cx.props.is_attachable {
                rsx!(
                    Attach {
                        on_click: move |_| {}
                    }
                )
            }
            MessageInput {
                message: "{message_field}",
                placeholder: cx.props.placeholder,
                itype: cx.props.message_type,
                on_input: move |event: FormEvent| {
                    message_field.set(event.value.clone());
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
            
        }
      }
    })
}
