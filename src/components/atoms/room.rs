use dioxus::prelude::*;

use crate::components::atoms::Avatar;
#[derive(Debug, Clone)]
pub struct RoomItem {
    pub avatar_uri: Option<String>,
    pub id: String,
    pub name: String,
    pub is_public: bool,
}

#[derive(Props)]
pub struct RoomViewProps<'a> {
    displayname: &'a str,
    #[props(!optional)]
    avatar_uri: Option<String>,
    description: Option<&'a str>,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn RoomView<'a>(cx: Scope<'a, RoomViewProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
      div {
        class: "room-view",
        onclick: move |event| cx.props.on_click.call(event),

        Avatar {
          name: String::from(cx.props.displayname),
          size: 60,
          uri: cx.props.avatar_uri.clone()
        }
        article {
          p {
            class: "room-view__title",
            "{cx.props.displayname}"
          }
          p {
            class: "room-view__message",
            span {
              "{cx.props.description.unwrap_or(&String::new())}"
            }
          }
        }
      }
    })
}
