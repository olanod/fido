use dioxus::prelude::{SvgAttributes, *};

use crate::components::atoms::{Close, Icon};

#[derive(Debug)]
pub enum HeaderCallOptions {
    CLOSE,
}

#[derive(Debug)]
pub struct HeaderEvent {
    pub value: HeaderCallOptions,
}

#[derive(Props)]
pub struct HeaderProps<'a> {
    text: &'a str,
    on_event: EventHandler<'a, HeaderEvent>,
}

pub fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element<'a> {
    let nav_style = r#"
        color: var(--text-1);
        display: flex;
        justify-content: space-between;
        align-items: center;
        position: absolute;
        width: inherit;
        height: 40px;
        padding: 0 var(--size-4);
        background: var(--surface-1);
        font-weight: 600;
        top: 0;
        font-size: var(--font-size-0)
    "#;

    let close_style = r#"
      cursor: pointer;
      background: transparent;
      border: 1px solid transparent;
    "#;

    cx.render(rsx!(
        nav {
          style: "{nav_style}",
          span {
            "{cx.props.text}"
          }
          button {
            style: "{close_style}",
            onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
            Icon {
              stroke: "#818898",
              icon: Close
          }
          }
      }
    ))
}
