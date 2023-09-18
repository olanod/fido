use dioxus::prelude::*;

use crate::components::atoms::{Edit, Icon, MenuHamburger};

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
    let header_style = r#"
        display: flex;
        justify-content: space-between;
        width: 100%;
        align-items: center;
        margin: 20px 0;
    "#;

    let title_style = r#"
        font-size: 18px;
    "#;

    let close_style = r#"
        cursor: pointer;
        background: transparent;
        border: 1px solid transparent;
        padding: 0;
    "#;

    cx.render(rsx!(
        section{
            style: "{header_style}",
            button {
                style: "{close_style}",
                onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
                Icon {
                  stroke: "#000000",
                  icon: MenuHamburger,
                  height: 30,
                  width: 30
                }
            }
            h2 {
                style: "{title_style}",
                "Chats"
            }
            button {
                style: "{close_style}",
                onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::EDIT })},
                Icon {
                    stroke: "#000000",
                    icon: Edit,
                    height: 30,
                    width: 30
                }
            }
        }
    ))
}
