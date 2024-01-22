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
    cx.render(rsx!(
        nav {
          class: "nav",
          button {
            class: "nav__cta",
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
            class: "nav__title",
            "{cx.props.text}"
          }
      }
    ))
}
