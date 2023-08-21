use dioxus::prelude::*;

use crate::components::atoms::{ArrowLeft, Avatar, Icon};

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
    #[props(!optional)]
    avatar_uri: Option<&'a String>,
    text: &'a str,
    on_event: EventHandler<'a, HeaderEvent>,
}

pub fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element<'a> {
    let nav_style = r#"
        color: var(--text-1);
        display: flex;
        gap: 0.5rem;
        align-items: center;
        position: absolute;
        width: inherit;
        padding: 1.25rem 0;
        background: var(--surface-1);
        font-weight: 600;
        top: 0;
        font-size: var(--font-size-0)
    "#;

    let close_style = r#"
      cursor: pointer;
      background: transparent;
      border: 1px solid transparent;
      padding: 0;
    "#;

    let title_style = r#"
      color: var(--text-loud);
      font-family: Inter;
      font-size: 18px;
      font-style: normal;
      font-weight: 500;
      line-height: 24px; /* 133.333% */
    "#;

    cx.render(rsx!(
        nav {
          style: "{nav_style}",
          button {
            style: "{close_style}",
            onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
            Icon {
              stroke: "#000000",
              icon: ArrowLeft,
              height: 24,
              width: 24
            }
          }
          Avatar {
            name: cx.props.text.clone(),
            size: 32,
            uri: cx.props.avatar_uri
          }
          span {
            style: "{title_style}",
            "{cx.props.text}"
          }
      }
    ))
}
