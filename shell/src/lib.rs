use dioxus::prelude::*;
use home::Home;

mod home;

const APPS: [&'static str; 6] = ["accounts", "contacts", "forum", "term", "settings", "help"];
const EXTERNAL: [&'static str; 3] = ["swap.cash", "flea.market", "go.delivery"];
const TRICKS: [&'static str; 2] = ["pay", "chat"];

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        fido::status { "⌗ Home" }
        Prompt { endpoint: "#" }
        section {
            id: "app",
            Home { apps: &APPS[..], external: &EXTERNAL[..], tricks: &TRICKS[..] },
        }
    })
}

#[derive(Props, PartialEq)]
struct PromptProps {
    endpoint: &'static str,
}

fn Prompt(cx: Scope<PromptProps>) -> Element {
    cx.render(rsx! {
        form {
            id: "prompt",
            action: "{cx.props.endpoint}",
            method: "GET",
            order: "99",
            fido::prompt { name: "q" }
        }
    })
}

fn FidoDialog(cx: Scope) -> Element {
    cx.render(rsx! {
        dialog { output {} }
    })
}

custom_elements! {
    fido {
        status();
        prompt(name);
        frame();
        grid(select);
    }
}
