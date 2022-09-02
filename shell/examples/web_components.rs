use dioxus::prelude::*;
use fido_shell::fido;

fn main() {
    dioxus_web::launch(app)
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Fido custom elements" }

        h2 { code { "<fido-prompt>" } }
        fido::prompt { "type here ..." }

        h2 { code { "<fido-pane>" } }
        fido::pane { "custom elments work!" }
    })
}
