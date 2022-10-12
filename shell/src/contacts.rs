use dioxus::prelude::*;

pub fn Contacts(cx: Scope) -> Element {
    cx.render(rsx! { h2 { "Contacts" }})
}
