use dioxus::prelude::*;

use crate::components::atoms::{avatar::Variant, Avatar};

#[derive(Props)]
pub struct SpaceProps<'a> {
    text: &'a str,
    #[props(!optional)]
    uri: Option<String>,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Space<'a>(cx: Scope<'a, SpaceProps<'a>>) -> Element<'a> {
    render!(rsx!(
        button {
            class: "button button--tertiary padding-reset",
            onclick: move |event| cx.props.on_click.call(event),
            Avatar {
                name: cx.props.text.to_string(),
                size: 50,
                uri: cx.props.uri.clone(),
                variant: Variant::SemiRound
            }
        }
    ))
}
