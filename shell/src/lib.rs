use dioxus::prelude::*;
use web_components::*;

const APPS: [&'static str; 8] = [
    "chat", "accounts", "contacts", "pay", "forum", "term", "settings", "help",
];
const EXTRA: [&'static str; 3] = ["swap.cash", "flea.market", "go.delivery"];

pub fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        fido::status { "⌗ Home" }
        Prompt { endpoint: "#" }
        section {
            id: "app",
            Home { apps: &APPS[..], extra: &EXTRA[..] },
        }
    })
}

#[derive(Props, PartialEq)]
struct HomeProps<'a> {
    apps: &'a [&'static str],
    extra: &'a [&'static str],
}

fn Home<'a>(cx: Scope<'a, HomeProps<'a>>) -> Element {
    let builtin_apps = cx.props.apps.iter().map(|n| rsx!(AppIcon { name: n }));
    let extra_apps = cx.props.extra.iter().map(|n| rsx!(AppIcon { name: n }));

    cx.render(rsx! {
        fido::grid {
            select: "none",
            builtin_apps,
            h4 { "extra" },
            extra_apps,
        }
    })
}

#[derive(Props, PartialEq)]
struct IcProps<'a> {
    name: &'a str,
}

fn AppIcon<'a>(cx: Scope<'a, IcProps<'a>>) -> Element<'a> {
    const IMG_DIR: &str = "./ic";
    let name = cx.props.name;
    let title = name.replace(".", " ");
    cx.render(rsx! {
        a {
            title: "{title}",
            href: "#",
            fido::frame {
                class: "box",
                img { image_rendering: "pixelated", src: "{IMG_DIR}/{name}.webp", alt: "{name}" }
            }
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

pub mod web_components {
    use super::custom_elements;

    custom_elements! {
        fido {
            status();
            prompt(name);
            frame();
            grid(select);
        }
    }
}
