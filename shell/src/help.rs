use dioxus::prelude::*;

pub fn Help(cx: Scope) -> Element {
    cx.render(rsx! { h2 { "Help" }})
}
