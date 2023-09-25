use dioxus::prelude::*;

use crate::components::atoms::{icon::Icon, Search, Send, Warning};

#[derive(Props)]
pub struct MessageInputProps<'a> {
    itype: Option<&'a str>,
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
    let error_container_style = if let Some(_) = cx.props.error {
        r#"
            box-shadow: 0px 0px 0px 1px #DF1C41, 0px 0px 0px 2px #FFF, 0px 0px 0px 3px rgba(223, 28, 65, 0.24), 0px 1px 2px 0px rgba(150, 19, 44, 0.32);
        "#
    } else {
        ""
    };

    cx.render(rsx!(
        section {
            style: r#"
                display: flex;
                flex-direction: column;
                gap: 4px;
                width: 100%
            "#,
            if let Some(value) = cx.props.label {
                rsx!(
                    label {
                        "{value}"
                    }
                )
            }
            div {
                style: "{error_container_style}",
                class: "input-wrapper",

                if cx.props.itype.unwrap_or("text").eq("search") {
                    rsx!(
                        Icon {
                            stroke: "#818898",
                            icon: Search
                        }
                    )
                }

                input {
                    r#type: cx.props.itype.unwrap_or("text"),
                    class: "input",
                    value: cx.props.message,
                    placeholder: "{cx.props.placeholder}",
                    oninput: move |event| cx.props.on_input.call(event),
                    onkeypress: move |event| cx.props.on_keypress.call(event)
                }

                if cx.props.message.len() > 0 && !cx.props.itype.unwrap_or("text").eq("search") {
                    rsx!(
                        button {
                            class: "input__cta",
                            onclick: move |event| cx.props.on_click.call(event),
                            Icon {
                                stroke: "#818898",
                                icon: Send
                            }
                        }
                    )
                }
            }
            if let Some(error) = cx.props.error {
                let error_style = r#"
                    display: flex;
                    gap: 2px;
                    color: #DF1C41;
                    font-family: Inter;
                    font-size: 12px;
                    font-style: normal;
                    font-weight: 400;
                    line-height: 16px; 
                    align-items: center;
                    padding-top: 6px;
                "#;

                rsx!(
                    div {
                        style: "{error_style}",
                        Icon {
                            stroke: "#DF1C41",
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
