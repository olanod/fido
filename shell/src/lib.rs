use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Hello Fido!" }
        p { "Here your Fido companion will learn a trick or two 😉 " }
        fido::pane { "custom elments work!" }
    })
}

custom_elements! {
    fido {
        pane();
        prompt();
    }
}
