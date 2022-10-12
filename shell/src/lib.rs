use dioxus::prelude::*;
use dioxus_router::{Link, Redirect, Route, Router};

use accounts::Accounts;
use chat::Chat;
use contacts::Contacts;
use forum::Forum;
use help::Help;
use home::Home;
use settings::Settings;
use term::Term;

mod accounts;
mod chat;
mod contacts;
mod forum;
mod help;
mod home;
mod settings;
mod term;

const APPS: [&'static str; 6] = ["accounts", "contacts", "forum", "term", "settings", "help"];
const EXTERNAL: [&'static str; 3] = ["swap.cash", "flea.market", "go.delivery"];
const TRICKS: [&'static str; 2] = ["pay", "chat"];

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            fido::status { Link { to: "/home", "⌗ Home" } }
            Prompt { endpoint: "#" }
            section {
                id: "app",
                Route { to: "/home", Home { apps: &APPS[..], external: &EXTERNAL[..], tricks: &TRICKS[..] } }
                Route { to: "/accounts", Accounts {} }
                Route { to: "/contacts", Contacts {} }
                Route { to: "/forum", Forum {} }
                Route { to: "/help", Help {} }
                Route { to: "/term", Term {} }
                Route { to: "/settings", Settings {} }
            }
            Redirect { from: "", to: "/home" }
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
