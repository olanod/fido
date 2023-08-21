use dioxus::prelude::*;

use crate::components::atoms::{icon::Icon, Search, Send};

#[derive(Props)]
pub struct MessageInputProps<'a> {
    itype: Option<&'a str>,
    message: &'a str,
    placeholder: &'a str,
    on_input: EventHandler<'a, FormEvent>,
    on_keypress: EventHandler<'a, KeyboardEvent>,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn MessageInput<'a>(cx: Scope<'a, MessageInputProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        section {
            class: "input-wrapper",
            if let Some(itype) = cx.props.itype {
                if itype.eq("search") {
                    rsx!(
                    Icon {
                        stroke: "#818898",
                        icon: Search
                    })
                } else {
                    rsx!(
                        div{}
                    )
                }
            }

            input {
                r#type: cx.props.itype.unwrap_or("text"),
                class: "input",
                value: cx.props.message,
                placeholder: "{cx.props.placeholder}",
                oninput: move |event| cx.props.on_input.call(event),
                onkeypress: move |event| cx.props.on_keypress.call(event)
            }
            
            if cx.props.message.len() > 0 && !cx.props.itype.unwrap_or("text").eq("search"){
                rsx!(
                    button {
                        class: "input__cta",
                        onclick: move |event| {
                            cx.props.on_click.call(event);
                        },
                        Icon {
                            stroke: "#818898",
                            icon: Send
                        }
                    }
                )
            }
        }
    ))
}
