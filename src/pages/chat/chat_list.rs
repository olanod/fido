use std::{collections::HashMap, ops::Deref};

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use log::info;

use crate::{
    components::{
        atoms::{
            avatar::Variant, input::InputType, message::Messages, room::RoomItem, Avatar,
            MessageInput,
        },
        molecules::{
            rooms::{CurrentRoom, FormRoomEvent},
            RoomsList,
        },
        organisms::{chat::ActiveRoom, main::TitleHeaderMain},
    },
    hooks::use_client::use_client,
    pages::route::Route,
    services::matrix::matrix::{list_rooms_and_spaces, Conversations},
};

#[inline_props]
pub fn ChatList(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let client = use_client(cx).get();

    // use_shared_state_provider::<HashMap<CurrentRoom, Messages>>(cx, || HashMap::new());

    let current_room = use_shared_state::<CurrentRoom>(cx).expect("Unable to read current room");
    let room_tabs = use_ref::<HashMap<CurrentRoom, Messages>>(cx, || HashMap::new());

    let rooms = use_state::<Vec<RoomItem>>(cx, || Vec::new());
    let all_rooms = use_state::<Vec<RoomItem>>(cx, || Vec::new());
    let spaces = use_state::<HashMap<RoomItem, Vec<RoomItem>>>(cx, || HashMap::new());
    let rooms_to_list = use_ref::<Vec<RoomItem>>(cx, || Vec::new());
    let pattern = use_state(cx, String::new);
    let rooms_filtered = use_ref(cx, || Vec::new());
    let selected_space = use_ref::<String>(cx, || String::new());
    let messages = use_shared_state::<Messages>(cx).unwrap();
    let title_header =
        use_shared_state::<TitleHeaderMain>(cx).expect("Unable to read title header");

    let on_click_room = move |evt: FormRoomEvent| {
        // nav.push(Route::ChatRoom {
        //     name: evt.room.id.clone(),
        // });
        *current_room.write() = evt.room.clone();
        room_tabs.with_mut(|tabs| tabs.insert(evt.room, vec![]));
        messages.write().clear();
    };

    use_coroutine(cx, |_: UnboundedReceiver<bool>| {
        to_owned![
            client,
            rooms,
            spaces,
            rooms_to_list,
            rooms_filtered,
            all_rooms,
            selected_space,
            title_header
        ];

        async move {
            let Conversations {
                rooms: r,
                spaces: s,
            } = list_rooms_and_spaces(&client).await;

            rooms.set(r.clone());
            spaces.set(s.clone());

            s.iter().for_each(|space| {
                all_rooms.with_mut(|all_r| {
                    all_r.extend_from_slice(&space.1);
                })
            });

            all_rooms.with_mut(|all_r| {
                all_r.extend_from_slice(&r.clone());
            });

            rooms_to_list.set(r.clone());
            rooms_filtered.set(r);

            selected_space.set(String::from("Inicio"));
            title_header.write().title = String::from("Inicio");
        }
    });

    render! {
        section {
            class: "chat-list options",
            div {
                ul {
                    class: "chat-list__wrapper",
                    button {
                      class: "button button--tertiary padding-reset",
                      onclick: move |_| {
                          rooms_to_list.set(rooms.get().clone());
                          rooms_filtered.set(rooms.get().clone());
                          selected_space.set(String::from("Inicio"));
                          title_header.write().title = String::from("Inicio");
                      },
                      Avatar {
                          name: String::from("Inicio"),
                          size: 50,
                          uri: None,
                          variant: Variant::SemiRound
                      }
                    }
                    spaces.get().iter().map(|(space, value)|{
                        rsx!(
                            button {
                                class: "button button--tertiary padding-reset",
                                onclick: move |_| {
                                    rooms_to_list.set(value.clone());
                                    rooms_filtered.set(value.clone());
                                    selected_space.set(space.name.clone());
                                    title_header.write().title = space.name.clone();
                                },
                                Avatar {
                                    name: space.name.clone(),
                                    size: 50,
                                    uri: space.avatar_uri.clone(),
                                    variant: Variant::SemiRound
                                }
                            }
                        )
                    })
                }
            }

            if !rooms_to_list.read().is_empty() {
                rsx!(
                    div {
                        class: "chat-list__rooms",
                        MessageInput {
                            message: "{pattern}",
                            placeholder: "Buscar",
                            itype: InputType::Search,
                            error: None,
                            on_input: move |event: FormEvent| {
                                pattern.set(event.value.clone());

                                let default_rooms = all_rooms.get().iter().cloned().collect::<Vec<_>>();

                                if event.value.len() > 0 {
                                    let x = default_rooms
                                        .iter()
                                        .filter(|r| r.name.to_lowercase().contains(&event.value.to_lowercase()))
                                        .cloned()
                                        .collect::<Vec<_>>();

                                    rooms_filtered.set(x);
                                } else {
                                    rooms_filtered.set(rooms_to_list.read().clone())
                                }
                            },
                            on_keypress: move |_| {},
                            on_click: move |_| {}
                        }

                        RoomsList {
                            rooms: rooms_filtered.read().clone(),
                            on_submit: on_click_room
                        }
                    }
                )
            }

            if !current_room.read().name.is_empty() {
                let room_tabs = room_tabs.read().clone();
                rsx!(
                    section {
                        class: "chat-list__active-room",
                        ActiveRoom {}
                    }
                )
            }
        }
    }
}
