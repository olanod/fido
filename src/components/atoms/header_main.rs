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

    let header_style = r#"
        display: flex;
        justify-content: space-between;
        width: 100%;
        align-items: center;
        margin: 20px 0;
        padding: 0 10px;
    "#;

    let title_style = r#"
        color: var(--text-1);
        font-family: Inter;
        font-size: 18px;
        font-style: normal;
        font-weight: 500;
        line-height: 24px;
    "#;

    let close_style = r#"
        cursor: pointer;
        background: transparent;
        border: 1px solid transparent;
        padding: 0;
        height: 30px;
        display: flex;
        justify-content: center;
        align-items: center;
    "#;

    cx.render(rsx!(
        section{
            style: "{header_style}",
            div {
                style: r#"
                    display: flex;
                    gap: 24px;
                    align-items: center;
                "#,
                button {
                    style: "{close_style}",
                    onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
                    Icon {
                      stroke: "var(--text-1)",
                      icon: MenuHamburger,
                      height: 30,
                      width: 30
                    }
                }
                h2 {
                    style: "{title_style}",
                    "{title_header.read().title}"
                }
            }
            button {
                style: "{close_style}",
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
