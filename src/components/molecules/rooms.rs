use dioxus::prelude::*;

use crate::components::atoms::{room::RoomItem, MessageInput, RoomView, input::InputType};

#[derive(Clone, Debug)]
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
    rooms: &'a Vec<RoomItem>,
    on_submit: EventHandler<'a, FormRoomEvent>,
}

pub fn RoomsList<'a>(cx: Scope<'a, RoomsListProps<'a>>) -> Element<'a> {
    let pattern = use_state(cx, String::new);
    let rooms_filtered = use_state(cx, || {
        cx.props
            .rooms
            .iter()
            // .filter(|r| r.is_public)
            .cloned()
            .collect::<Vec<_>>()
    });

    let rooms_style = r#"
        display: flex;
        gap: 10px;
        flex-direction: column;
    "#;

    cx.render(rsx! {
        section {
            style: "{rooms_style}",
            MessageInput {
                message: "{pattern}",
                placeholder: "Buscar",
                itype: InputType::Search,
                error: None,
                on_input: move |event: FormEvent| {
                    pattern.set(event.value.clone());

                    let default_rooms = cx.props.rooms.iter().filter(|r| r.is_public).cloned().collect::<Vec<_>>();

                    if event.value.len() > 0 {
                        let x = default_rooms
                            .iter()
                            .filter(|r| r.name.to_lowercase().contains(&event.value.to_lowercase()))
                            .cloned()
                            .collect::<Vec<_>>();

                        rooms_filtered.set(x);
                    } else {
                        rooms_filtered.set(default_rooms)
                    }
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
            div{
                class:"rooms-list",
                rooms_filtered.get().iter().map(|room| {
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
        }
    })
}
