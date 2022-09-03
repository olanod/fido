use dioxus::prelude::*;
use fido_shell::web_components::fido;

fn main() {
    dioxus_web::launch(app)
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Fido custom elements" }

        h2 { code { "<fido-window>" } }
        fido::window { "custom elments work!" }

        h2 { code { "<fido-prompt>" } }
        fido::prompt { "type here ..." }
    })
}
