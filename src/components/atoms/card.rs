use crate::components::atoms::{Close, Icon};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct CardProps {
    file: String,
    on_click: EventHandler<MouseEvent>,
}

pub fn Card(props: CardProps) -> Element {
    rsx!(
        section { class: "card", onclick: move |event| props.on_click.call(event),
            div { class: "card-container",
                img { class: "card__media", src: "{props.file}" }

                button {
                    class: "card__cta",
                    onclick: move |event| { props.on_click.call(event) },
                    Icon { stroke: "var(--text-1)", icon: Close }
                }
            }
        }
    )
}
