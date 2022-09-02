mod fido_elements;

use dioxus::prelude::*;
pub use fido_elements::fido;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello Fido!" }
        p { "Here your Fido companion will learn a trick or two 😉 " }
        fido::pane { "custom elments work!" }
    })
}
