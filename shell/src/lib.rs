use dioxus::prelude::*;
use web_components::*;

const APPS: [&'static str; 11] = [
    "chat",
    "accounts",
    "contacts",
    "pay",
    "forum",
    "term",
    "settings",
    "help",
    "swap.cash",
    "flea.market",
    "go.delivery",
];

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Prompt { endpoint: "#" }
        fido::frame {
            style: "flex: 1; --padding: var(--font-size-fluid-0)",
            class: "simple",
            Home { apps: &APPS[..] }
        }
        fido::status { "⌗ Home" }
    })
}

#[derive(Props, PartialEq)]
struct HomeProps {
    apps: &'static [&'static str],
}

fn Home(cx: Scope<HomeProps>) -> Element {
    let app_list = cx.props.apps.iter().map(|name| {
        const IMG_DIR: &str = "./ic";
        rsx! {
            a {
                href: "#",
                fido::frame {
                    class: "box",
                    title: "{name}",
                    img { image_rendering: "pixelated", src: "{IMG_DIR}/{name}.webp", alt: "{name}" }
                },
                "{name}"
            }
        }
    });

    cx.render(rsx! {
        fido::grid {
            margin: "auto 0",
            select: "none",
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
            grid(select);
        }
    }
}
