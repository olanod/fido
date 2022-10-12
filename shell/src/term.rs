use dioxus::prelude::*;

pub fn Term(cx: Scope) -> Element {
    cx.render(rsx! { h2 { "Term" }})
}
