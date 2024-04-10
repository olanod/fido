use crate::components::atoms::Avatar;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct UserProps {
    display_name: String,
    avatar_uri: Option<String>,
    on_click: EventHandler<MouseEvent>,
}

pub fn UserProfile(props: UserProps) -> Element {
    rsx! {
        div {
            class: "user",
            onclick: move |event| { props.on_click.call(event) },
            Avatar { name: props.display_name.to_string(), size: 36, uri: props.avatar_uri }
            article {
                section { class: "user__wrapper", span { class: "user__content", "{props.display_name}" } }
            }
        }
    }
}
