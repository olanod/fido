use dioxus::prelude::*;

use crate::components::atoms::Button;

#[derive(Props)]
pub struct GuestProps<'a> {
    description: &'a str,
    cta: &'a str,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Guest<'a>(cx: Scope<'a, GuestProps<'a>>) -> Element<'a> {
    render!(rsx!(
        section {
            class: "guest",
            span {
                "{cx.props.description}",
            }
            div {
                class: "guest__cta",
                Button {
                    text: "{cx.props.cta}",
                    on_click: move |event| {
                        cx.props.on_click.call(event)
                    },
                    status: None
                }
            }
        }
    ))
}
