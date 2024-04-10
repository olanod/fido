use dioxus::prelude::*;

use crate::components::atoms::{avatar::Variant, Avatar};

#[derive(PartialEq, Props, Clone)]
pub struct SpaceProps {
    text: String,
    #[props(!optional)]
    uri: Option<String>,
    on_click: EventHandler<MouseEvent>,
}

pub fn Space(props: SpaceProps) -> Element {
    rsx!(
        button {
            class: "button button--tertiary padding-reset",
            onclick: move |event| props.on_click.call(event),
            Avatar {
                name: props.text.to_string(),
                size: 50,
                uri: props.uri.clone(),
                variant: Variant::SemiRound
            }
        }
    )
}
