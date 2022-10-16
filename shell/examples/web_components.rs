use dioxus::prelude::*;
use fido_shell::fido_elements::*;

fn main() {
    dioxus_web::launch(app)
}

fn app(cx: Scope) -> Element {
    render! {
        h1 { "Fido custom elements" }

        h2 { code { "<fido-frame>" } }
        frame { "custom elments work!" }

        h2 { code { "<fido-prompt>" } }
        prompt { "type here ..." }
    }
}
