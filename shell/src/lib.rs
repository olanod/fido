use dioxus::prelude::*;
use dioxus_router::{Link, Redirect, Route, Router};

use communities::Communities;
use contacts::Contacts;
use finances::Finances;
use home::Home;
use knowledge::KnowledgeBase;
use news::News;
use profile::Profile;
use purchases::Purchases;
use settings::Settings;
use terminal::Term;

mod communities;
mod contacts;
mod finances;
mod home;
mod knowledge;
mod news;
mod profile;
mod purchases;
mod settings;
mod terminal;

const APPS: [&'static str; 9] = [
    "communities",
    "contacts",
    "finances",
    "knowledge_base",
    "news",
    "profile",
    "purchases",
    "settings",
    "terminal",
];
const EXTERNAL: [&'static str; 3] = ["swap.cash", "flea.market", "go.delivery"];
const TRICKS: [&'static str; 3] = ["pay", "message", "capture"];

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            fido::status { Link { to: "/home", "⌗ Home" } }
            Prompt { endpoint: "#" }
            section {
                id: "app",
                Route { to: "/home", Home { apps: &APPS[..], external: &EXTERNAL[..], tricks: &TRICKS[..] } }
                Route { to: "/news", News {} }
                Route { to: "/finances", Finances {} }
                Route { to: "/profile", Profile {} }
                Route { to: "/communities", Communities {} }
                Route { to: "/contacts", Contacts {} }
                Route { to: "/purchases", Purchases {} }
                Route { to: "/knowledge-base", KnowledgeBase {} }
                Route { to: "/terminal", Term {} }
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
