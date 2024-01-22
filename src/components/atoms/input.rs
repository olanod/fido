use dioxus::prelude::*;

use crate::components::atoms::{icon::Icon, Search, Send, Warning};

#[derive(Clone)]
pub enum InputType {
    Text,
    Message,
    Search,
    Password,
}

#[derive(Props)]
pub struct MessageInputProps<'a> {
    #[props(default = InputType::Text)]
    itype: InputType,
    message: &'a str,
    placeholder: &'a str,
    #[props(!optional)]
    error: Option<&'a String>,
    label: Option<&'a str>,
    on_input: EventHandler<'a, FormEvent>,
    on_keypress: EventHandler<'a, KeyboardEvent>,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn MessageInput<'a>(cx: Scope<'a, MessageInputProps<'a>>) -> Element<'a> {
    let input_error_container = if let Some(_) = cx.props.error {
        "input--error-container"
    } else {
        ""
    };

    let input_type = match cx.props.itype {
        InputType::Text => "text",
        InputType::Search => "search",
        InputType::Message => "text",
        InputType::Password => "password",
    };

    cx.render(rsx!(
        section {
            class: "input__wrapper",
            if let Some(value) = cx.props.label {
                rsx!(
                    label {
                        class: "input__label",
                        "{value}"
                    }
                )
            }
            div {
                class: "input-wrapper input_error_container",
                match cx.props.itype {
                    InputType::Search => {
                        rsx!(
                            Icon {
                                stroke: "var(--icon-subdued)",
                                icon: Search
                            }
                        )
                    }
                    _ => {
                        rsx!(div {})
                    }
                }

                input {
                    r#type: "{input_type}",
                    class: "input",
                    value: cx.props.message,
                    placeholder: "{cx.props.placeholder}",
                    oninput: move |event| cx.props.on_input.call(event),
                    onkeypress: move |event| cx.props.on_keypress.call(event)
                }

                if cx.props.message.len() > 0 {
                   match cx.props.itype {
                        InputType::Message => rsx!(
                            button {
                                class: "input__cta",
                                onclick: move |event| cx.props.on_click.call(event),
                                Icon {
                                    stroke: "var(--icon-subdued)",
                                    icon: Send
                                }
                            }
                        ),
                        _ => rsx!(div {})
                   }
                }
            }
            if let Some(error) = cx.props.error {
                rsx!(
                    div {
                        class: "input--error",
                        Icon {
                            stroke: "var(--secondary-red-100)",
                            height: 16,
                            width: 16,
                            icon: Warning
                        }
                        "{error}"
                    }
                )
            }
        }
    ))
}
