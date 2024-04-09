use dioxus::prelude::*;

use crate::components::atoms::{room::RoomItem, RoomView, RoomViewSkeleton};

#[derive(Clone, Debug, PartialEq, Hash, Eq, Default)]
pub struct CurrentRoom {
    pub id: String,
    pub name: String,
    pub avatar_uri: Option<String>,
}

#[derive(Debug)]
pub struct FormRoomEvent {
    pub room: CurrentRoom,
}

#[derive(PartialEq, Props, Clone)]
pub struct RoomsListProps {
    rooms: Vec<RoomItem>,
    is_loading: bool,
    #[props(default = false)]
    wrap: bool,
    on_submit: EventHandler<FormRoomEvent>,
}

pub fn RoomsList(props: RoomsListProps) -> Element {
    let rooms_list_skeleton = if props.rooms.is_empty() {
        "rooms-list--skeleton"
    } else {
        ""
    };
    let room_list_wrap = if props.wrap { "room-list--wrap" } else { "" };

    rsx! {
        section { class: "rooms-list {room_list_wrap} {rooms_list_skeleton} fade-in",
            if !props.rooms.is_empty() {
                for room in props.rooms.clone() {
                    RoomView {
                        displayname: room.name.as_str(),
                        avatar_uri: room.avatar_uri.clone(),
                        description: "",
                        wrap: props.wrap,
                        on_click: move |_| {
                            props
                                .on_submit
                                .call(FormRoomEvent {
                                    room: CurrentRoom {
                                        id: room.id.clone(),
                                        name: room.name.clone(),
                                        avatar_uri: room.avatar_uri.clone(),
                                    },
                                })
                        }
                    }
                }
            } else if props.is_loading {
                {(0..20).map(|i| {
                    rsx!(
                        RoomViewSkeleton {
                            key: "{i}"
                        }
                    )
                })}
            }
        }
    }
}
