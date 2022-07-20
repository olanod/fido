use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello Fido!" }
        p { "Here your Fido companion will be learn a trick or two 😉 " }
    })
}
