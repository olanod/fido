use crate::fido;
use dioxus::prelude::*;
use dioxus_router::Link;

const IMG_DIR: &'static str = "./ic";

#[derive(Props, PartialEq)]
pub struct HomeProps<'a> {
    apps: &'a [&'static str],
    external: &'a [&'static str],
    tricks: &'a [&'static str],
}

pub fn Home<'a>(cx: Scope<'a, HomeProps<'a>>) -> Element {
    let builtin_apps = cx.props.apps.iter().map(|n| rsx!(AppIcon { name: n }));
    let tricks = cx.props.tricks.iter().map(|n| rsx!(Trick { name: n }));
    let external_apps = cx.props.external.iter().map(|name| {
        rsx! {
            a {
                href: "https://{name}", class: "app-ic", title: "{name}",
                fido::frame { class: "box m", img { src: "{IMG_DIR}/{name}.webp", alt: "{name}" } }
            }
        }
    });

    cx.render(rsx! {
        fido::grid {
            select: "none",
            builtin_apps,
            h4 { "external" },
            external_apps,
            h4 { "quick tricks" },
            tricks,
        }
    })
}

#[derive(Props, PartialEq)]
struct IcProps<'a> {
    name: &'a str,
}

fn AppIcon<'a>(cx: Scope<'a, IcProps<'a>>) -> Element<'a> {
    let name = cx.props.name;
    let title = name.replace("_", " ");
    cx.render(rsx! {
        Link {
            to: "/{name}",
            class: "app-ic",
            title: "{title}",
            fido::frame { class: "box m", img { src: "{IMG_DIR}/{name}.webp", alt: "{name}" } }
        }
    })
}

fn Trick<'a>(cx: Scope<'a, IcProps<'a>>) -> Element<'a> {
    let name = cx.props.name;
    cx.render(rsx! {
        button {
            class: "trick",
            fido::frame {
                class: "card m",
                style: "--frame-bg: url({IMG_DIR}/{name}.webp)",
                i { "{name}" }
            }
        }
    })
}
