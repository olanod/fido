use dioxus::prelude::*;

use crate::components::atoms::{room::RoomItem, RoomView};

#[derive(Debug)]
pub struct CurrentRoom {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct FormRoomEvent {
    pub room: CurrentRoom,
}

#[derive(Props)]
pub struct RoomsListProps<'a> {
    rooms: &'a Vec<RoomItem>,
    on_submit: EventHandler<'a, FormRoomEvent>,
}

pub fn RoomsList<'a>(cx: Scope<'a, RoomsListProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div{
            class:"list",
            cx.props.rooms.iter().map(|room| {
                rsx!(RoomView {
                    key: "{room.id}",
                    room_avatar_uri: room.avatar_uri.as_ref(),
                    room_id: room.id.as_str(),
                    room_name: room.name.as_str() ,
                    on_click: move |_| {
                        cx.props.on_submit.call(FormRoomEvent { room: CurrentRoom { id: room.id.clone(), name: room.name.clone() } })
                    }
                })
            })
        }
    })
}
