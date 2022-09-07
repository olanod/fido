use dioxus::prelude::*;
use web_components::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello Fido!" }
        fido::window {
            p { "Here your Fido companion will learn a trick or two 😉 " }
        }
        fido::prompt {}
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
