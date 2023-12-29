use dioxus::prelude::*;

use crate::components::atoms::{header_main::HeaderCallOptions, ArrowLeft, Icon};

use super::header_main::HeaderEvent;

#[derive(Props)]
pub struct HeaderProps<'a> {
    avatar_element: Option<Element<'a>>,
    text: &'a str,
    on_event: EventHandler<'a, HeaderEvent>,
}

pub fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element<'a> {
    let nav_style = r#"
        color: var(--text-1);
        display: flex;
        gap: 0.5rem;
        align-items: center;
        width: 100%;
        padding: 1rem 0;
        background: var(--background);
        font-weight: 600;
        font-size: var(--font-size-0)
    "#;

    let close_style = r#"
      cursor: pointer;
      background: transparent;
      border: 1px solid transparent;
      padding: 0;
    "#;

    let title_style = r#"
      color: var(--text-1);
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
              stroke: "var(--text-1)",
              icon: ArrowLeft,
              height: 24,
              width: 24
            }
          }
          if let Some(element) = &cx.props.avatar_element {
            rsx!(
              element
            )
          }
          span {
            style: "{title_style}",
            "{cx.props.text}"
          }
      }
    ))
}
