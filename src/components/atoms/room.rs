use dioxus::prelude::*;

use crate::components::atoms::Avatar;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RoomItem {
    pub avatar_uri: Option<String>,
    pub id: String,
    pub name: String,
    pub is_public: bool,
    pub is_direct: bool,
}

#[derive(PartialEq, Props, Clone)]
pub struct RoomViewProps {
    displayname: String,
    #[props(!optional)]
    avatar_uri: Option<String>,
    description: Option<String>,
    #[props(default = false)]
    wrap: bool,
    on_click: EventHandler<MouseEvent>,
}

pub fn RoomView(props: RoomViewProps) -> Element {
    let description = props.description.unwrap_or("".to_owned());
    let room_view_wrap = if props.wrap { "room-view--wrap" } else { "" };

    rsx! {
        div {
            class: "room-view {room_view_wrap} fade-in",
            onclick: move |event| props.on_click.call(event),

            Avatar {
                name: props.displayname.clone(),
                size: 60,
                uri: props.avatar_uri.clone()
            }
            article {
                p { class: "room-view__title", "{props.displayname}" }
                p { class: "room-view__message", span { "{description}" } }
            }
        }
    }
}
