use dioxus::prelude::*;

use crate::components::atoms::Avatar;
#[derive(Debug)]
pub struct RoomItem {
    pub avatar_uri: Option<String>,
    pub id: String,
    pub name: String,
}

#[derive(Props)]
pub struct RoomViewProps<'a> {
    #[props(!optional)]
    room_avatar_uri: Option<&'a String>,
    room_name: &'a str,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn RoomView<'a>(cx: Scope<'a, RoomViewProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
      button {
        class: "room-view",
        onclick: move |event| cx.props.on_click.call(event),

        Avatar {
          name: cx.props.room_name.clone(),
          size: 24,
          uri: cx.props.room_avatar_uri
        }
        span {
          class: "room-view__title",
          "{cx.props.room_name}"
        }
      }
    })
}
