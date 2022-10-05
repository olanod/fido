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
        section {
            id: "app",
            Home { apps: &APPS[..] },
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
        let title = name.replace(".", " ");
        rsx! {
            a {
                title: "{title}",
                href: "#",
                fido::frame {
                    class: "box",
                    img { image_rendering: "pixelated", src: "{IMG_DIR}/{name}.webp", alt: "{name}" }
                }
            }
        }
    });

    cx.render(rsx! {
        fido::grid {
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

fn FidoDialog(cx: Scope<HomeProps>) -> Element {
    cx.render(rsx! {
        dialog { output {} }
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
