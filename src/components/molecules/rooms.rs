use dioxus::prelude::*;

use crate::components::atoms::{room::RoomItem, RoomView, RoomViewSkeleton};

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
    is_loading: bool,
    on_submit: EventHandler<'a, FormRoomEvent>,
}

pub fn RoomsList<'a>(cx: Scope<'a, RoomsListProps<'a>>) -> Element<'a> {
    let rooms_list_skeleton = if cx.props.rooms.len() > 0 {
        ""
    } else {
        "rooms-list--skeleton"
    };

    cx.render(rsx! {
        section {
            class:"rooms-list {rooms_list_skeleton} fade-in",
            if cx.props.rooms.len() > 0 {
                rsx!(cx.props.rooms.iter().map(|room| {
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
                }))
            } else if cx.props.is_loading {
                rsx!(
                    (0..20).map(|_| {
                        rsx!(
                            RoomViewSkeleton {}
                        )
                    })
                )
            } else {
                rsx!(div{})
            }
        }
    })
}
