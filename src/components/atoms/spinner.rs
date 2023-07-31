use dioxus::prelude::*;

pub fn Spinner(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
          class: "spinner-dual-ring"
        }
    ))
}
