use dioxus::prelude::*;
use web_components::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        fido::status { "⌗ Home" }
        main {
            fido::app {  }
        }
        form {
            action: "#input",
            method: "GET",
            fido::prompt { name: "q" }
        }
    })
}

pub mod web_components {
    use super::custom_elements;

    custom_elements! {
        fido {
            status();
            prompt(name);
            app();
        }
    }
}
