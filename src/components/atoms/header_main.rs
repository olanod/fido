use dioxus::prelude::*;

use crate::components::{
    atoms::{Edit, Icon, MenuHamburger},
    organisms::main::TitleHeaderMain,
};

#[derive(Debug)]
pub enum HeaderCallOptions {
    CLOSE,
    EDIT,
}

#[derive(Debug)]
pub struct HeaderEvent {
    pub value: HeaderCallOptions,
}

#[derive(PartialEq, Props, Clone)]
pub struct HeaderProps {
    on_event: EventHandler<HeaderEvent>,
}

pub fn HeaderMain(props: HeaderProps) -> Element {
    let title_header = consume_context::<Signal<TitleHeaderMain>>();

    rsx!(
        section { class: "header",
            div { class: "header__content",
                button {
                    class: "header__cta",
                    onclick: move |_| {
                        props
                            .on_event
                            .call(HeaderEvent {
                                value: HeaderCallOptions::CLOSE,
                            })
                    },
                    Icon { stroke: "var(--text-1)", icon: MenuHamburger, height: 30, width: 30 }
                }
                h2 { class: "header__title", "{title_header.read().title}" }
            }
            button {
                class: "header__cta",
                onclick: move |_| {
                    props
                        .on_event
                        .call(HeaderEvent {
                            value: HeaderCallOptions::EDIT,
                        })
                },
                Icon { stroke: "var(--text-1)", icon: Edit, height: 30, width: 30 }
            }
        }
    )
}
