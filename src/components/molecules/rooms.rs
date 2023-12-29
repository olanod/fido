use dioxus::prelude::*;

use crate::components::atoms::{input::InputType, room::RoomItem, MessageInput, RoomView};

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct CurrentRoom {
    pub id: String,
    pub name: String,
    pub avatar_uri: Option<String>,
}

#[derive(Debug)]
pub struct FormRoomEvent {
    pub room: CurrentRoom,
}

#[derive(Props)]
pub struct RoomsListProps<'a> {
    rooms: Vec<RoomItem>,
    on_submit: EventHandler<'a, FormRoomEvent>,
}

pub fn RoomsList<'a>(cx: Scope<'a, RoomsListProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        section {
            class:"rooms-list",
            cx.props.rooms.iter().map(|room| {
                rsx!(RoomView {
                    key: "{room.id}",
                    displayname: room.name.as_str(),
                    avatar_uri: room.avatar_uri.clone(),
                    description: "",
                    on_click: move |_| {
                        cx.props.on_submit.call(FormRoomEvent {
                            room: CurrentRoom {
                                id: room.id.clone(),
                                name: room.name.clone(),
                                avatar_uri: room.avatar_uri.clone(),
                            },
                        })
                    }
                })
            })
        }
    })
}
