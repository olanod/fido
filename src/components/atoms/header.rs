use dioxus::prelude::*;

use crate::components::atoms::{header_main::HeaderCallOptions, ArrowLeft, Icon};

use super::header_main::HeaderEvent;

#[derive(PartialEq, Props, Clone)]
pub struct HeaderProps {
    avatar_element: Option<Element>,
    menu: Option<Element>,
    text: String,
    on_event: EventHandler<HeaderEvent>,
}

pub fn Header(props: HeaderProps) -> Element {
    rsx!(
        nav { class: "nav",
            div { class: "nav-wrapper",
                button {
                    class: "nav__cta",
                    onclick: move |_| {
                        props
                            .on_event
                            .call(HeaderEvent {
                                value: HeaderCallOptions::CLOSE,
                            })
                    },
                    Icon { stroke: "var(--text-1)", icon: ArrowLeft, height: 24, width: 24 }
                }
                { props.avatar_element.map(|e| rsx!({e})) },
                span { class: "nav__title", "{props.text}" }
            }
            { props.menu.map(|e| rsx!({e})) }
        }
    )
}
