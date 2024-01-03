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

#[derive(Props)]
pub struct HeaderProps<'a> {
    on_event: EventHandler<'a, HeaderEvent>,
}

pub fn HeaderMain<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element<'a> {
    let title_header =
        use_shared_state::<TitleHeaderMain>(cx).expect("Unable to read title header");

    cx.render(rsx!(
        section{
            class: "header",
            div {
                class: "header__content",
                button {
                    class: "header__cta",
                    onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
                    Icon {
                      stroke: "var(--text-1)",
                      icon: MenuHamburger,
                      height: 30,
                      width: 30
                    }
                }
                h2 {
                    class: "header__title",
                    "{title_header.read().title}"
                }
            }
            button {
                class: "header__cta",
                onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::EDIT })},
                Icon {
                    stroke: "var(--text-1)",
                    icon: Edit,
                    height: 30,
                    width: 30
                }
            }
        }
    ))
}
