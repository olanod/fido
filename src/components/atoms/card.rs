use crate::components::atoms::{Close, Icon};
use dioxus::prelude::*;

#[derive(Props)]
pub struct CardProps<'a> {
    file: &'a str,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        section {
            class: "card",
            onclick: move |event| cx.props.on_click.call(event),
            div {
                class: "card-container",
                img {
                    class: "card__media",
                    src: "{cx.props.file}"
                }

                button {
                    class: "card__cta",
                    onclick: move |event| {cx.props.on_click.call(event)},
                    Icon {
                        stroke: "var(--text-1)",
                        icon: Close
                    }
                }
            }
        }
    ))
}
