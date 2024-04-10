use dioxus::prelude::*;

use crate::components::atoms::Button;

#[derive(PartialEq, Props, Clone)]
pub struct GuestProps {
    description: String,
    cta: String,
    on_click: EventHandler<MouseEvent>,
}

pub fn Guest(props: GuestProps) -> Element {
    rsx!(
        section { class: "guest",
            span { "{props.description}" }
            div { class: "guest__cta",
                Button {
                    text: "{props.cta}",
                    on_click: move |event| { props.on_click.call(event) },
                    status: None
                }
            }
        }
    )
}
