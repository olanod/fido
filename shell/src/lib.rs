use dioxus::prelude::*;
use dioxus_router::{Link, Redirect, Route, Router};

use communities::Communities;
use contacts::Contacts;
use funds::Funds;
use home::Home;
use news::News;
use profile::Profile;
use purchases::Purchases;
use settings::Settings;
use terminal::Term;
use wiki::Wiki;

mod communities;
mod contacts;
mod funds;
mod home;
mod news;
mod profile;
mod purchases;
mod settings;
mod terminal;
mod wiki;

const APPS: [&'static str; 9] = [
    "communities",
    "contacts",
    "funds",
    "news",
    "profile",
    "purchases",
    "settings",
    "terminal",
    "wiki",
];
const EXTERNAL: [&'static str; 3] = ["swap.cash", "flea.market", "go.delivery"];
const TRICKS: [&'static str; 3] = ["pay", "message", "capture"];

pub mod fido_elements {
    use dioxus::prelude::*;

    custom_elements! {
        status("fido-status",);
        prompt("fido-prompt", name);
        frame("fido-frame",);
        grid("fido-grid", select);
    }
}

pub fn app(cx: Scope) -> Element {
    use fido_elements::*;
    render! {
        Router {
            status { Link { to: "/home", "⌗ Home" } }
            Prompt { endpoint: "#" }
            section {
                id: "app",
                Route { to: "/home", Home { apps: &APPS[..], external: &EXTERNAL[..], tricks: &TRICKS[..] } }
                Route { to: "/news", News {} }
                Route { to: "/funds", Funds {} }
                Route { to: "/profile", Profile {} }
                Route { to: "/communities", Communities {} }
                Route { to: "/contacts", Contacts {} }
                Route { to: "/purchases", Purchases {} }
                Route { to: "/wiki", Wiki {} }
                Route { to: "/terminal", Term {} }
                Route { to: "/settings", Settings {} }
            }
            Redirect { from: "", to: "/home" }
        }
    }
}

#[derive(Props, PartialEq)]
struct PromptProps {
    endpoint: &'static str,
}

fn Prompt(cx: Scope<PromptProps>) -> Element {
    use fido_elements::*;
    render! {
        form {
            id: "prompt",
            action: "{cx.props.endpoint}",
            method: "GET",
            order: "99",
            prompt { name: "q" }
        }
    }
}

fn FidoDialog(cx: Scope) -> Element {
    render! {
        dialog { output {} }
    }
}
