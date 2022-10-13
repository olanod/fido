use dioxus::prelude::*;

pub fn News(cx: Scope) -> Element {
    cx.render(rsx! { h2 { "News" }})
}
