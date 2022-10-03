use dioxus::prelude::*;
use web_components::*;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Prompt { endpoint: "#" }
        fido::frame {
            style: "--padding: var(--font-size-fluid-0); --frame: none;",
            Home { apps: vec!["chat", "accounts", "contacts", "pay", "forum", "term", "settings", "help"] }
        }
        fido::status { "⌗ Home" }
    })
}

#[derive(Props, PartialEq)]
struct HomeProps {
    apps: Vec<&'static str>,
}

fn Home(cx: Scope<HomeProps>) -> Element {
    let app_list = cx.props.apps.iter().map(|name| {
        const IMG_DIR: &str = "./fido";
        rsx! {
            a {
                href: "#",
                class: "ic",
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
