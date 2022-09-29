use dioxus::prelude::*;
use web_components::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        fido::status { "⌗ Home" }
        fido::main {
            Home { apps: vec!["news", "accounts", "contacts", "help"] }
        }
        Prompt { endpoint: "#" }
    })
}

#[derive(Props, PartialEq)]
struct HomeProps {
    apps: Vec<&'static str>,
}

fn Home(cx: Scope<HomeProps>) -> Element {
    let app_list = cx.props.apps.iter().map(|name| {
        rsx! {
            figure {

                a { href: "#", fido::frame { "{name}" } }
                figcaption { "{name}" }
            }
        }
    });

    cx.render(rsx! {
        header {
            class: "hero",
            display: "flex",
            ul {
                fido::frame { class: "card", "Info 1" },
                fido::frame { class: "card", "Info 2" },
            }
        }
        section {
            id: "app-grid",
            display: "grid",
            app_list
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
            action: "{cx.props.endpoint}",
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
            main();
            prompt(name);
            frame();
            card(title);
            apps();
            app(ic);
        }
    }
}
