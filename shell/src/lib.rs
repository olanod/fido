mod fido_elements;

use dioxus::prelude::*;
use web_components::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello Fido!" }
        p { "Here your Fido companion will learn a trick or two 😉 " }
        fido::window { "custom elments work!" }
    })
}

pub mod web_components {
    use super::custom_elements;

    custom_elements! {
        fido {
            window();
            prompt();
        }
    }
}
