use dioxus::prelude::*;
use log::info;

use crate::components::{
    atoms::{icon::Icon, Search, Send, Warning},
    molecules::input_message::FormMessageEvent,
};

#[derive(Props)]
pub struct InputMoneyProps<'a> {
    message: &'a str,
    placeholder: &'a str,
    #[props(!optional)]
    error: Option<&'a String>,
    label: Option<&'a str>,
    on_input: EventHandler<'a, FormMessageEvent<f64>>,
    on_keypress: EventHandler<'a, KeyboardEvent>,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn InputMoney<'a>(cx: Scope<'a, InputMoneyProps<'a>>) -> Element<'a> {
    let error_container_style = if let Some(_) = cx.props.error {
        r#"
            box-shadow: 0px 0px 0px 1px var(--secondary-red-100), 0px 0px 0px 2px var(--background-white), 0px 0px 0px 3px rgba(223, 28, 65, 0.24), 0px 1px 2px 0px rgba(150, 19, 44, 0.32);
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
                        class: "input__label",
                        "{value}"
                    }
                )
            }
            div {
                style: "
                    {error_container_style}
                    width: 100%;
                    padding: 0.75rem 10px;
                    display: flex;
                    gap: 0.5rem;
                    align-items: center;
                ",

                input {
                    r#type: "number",
                    class: "input",
                    style: "
                        color: var(--secondary-red);
                        font-family: Inter;
                        font-size: 52px;
                        font-style: normal;
                        font-weight: 500;
                        line-height: 90%;
                        letter-spacing: -1.04px;
                        text-align: center;
                    ",
                    value: cx.props.message,
                    placeholder: "{cx.props.placeholder}",
                    oninput: move |event: Event<FormData>| {
                        info!("event from input money {:#?}", event);

                        let value: f64 = match event.data.value.parse::<f64>() {
                            Ok(v) => v,
                            _ => 0.0
                        };

                        info!("converted value to f64 {value}");

                        if value > 0.0 {
                            cx.props.on_input.call(FormMessageEvent { value: value });
                        }
                    },
                    onkeypress: move |event| cx.props.on_keypress.call(event)
                }
            }

            if let Some(error) = cx.props.error {
                let error_style = r#"
                    display: flex;
                    gap: 2px;
                    color: var(--secondary-red-100);
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
