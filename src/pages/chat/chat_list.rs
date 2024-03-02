use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

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
    hooks::{
        use_client::use_client,
        use_messages::use_messages,
        use_notification::use_notification,
        use_room::use_room,
        use_session::use_session,
    },
    services::matrix::matrix::{list_rooms_and_spaces, Conversations},
};

#[inline_props]
pub fn ChatList(cx: Scope) -> Element {
    let i18 = use_i18(cx);
    let client = use_client(cx).get();
    let session = use_session(cx);
    let notification = use_notification(cx);
    let room = use_room(cx);
    let messages = use_messages(cx);
    // use_shared_state_provider::<HashMap<CurrentRoom, Messages>>(cx, || HashMap::new());

    let room_tabs = use_ref::<HashMap<CurrentRoom, Messages>>(cx, || HashMap::new());

    let key_chat_list_home = translate!(i18, "chat.list.home");
    let key_chat_list_search = translate!(i18, "chat.list.search");
    let key_session_error_not_found = translate!(i18, "chat.session.error.not_found");

    let rooms = use_state::<Vec<RoomItem>>(cx, || Vec::new());
    let all_rooms = use_state::<Vec<RoomItem>>(cx, || Vec::new());
    let spaces = use_state::<HashMap<RoomItem, Vec<RoomItem>>>(cx, || HashMap::new());
    let rooms_to_list = use_ref::<Vec<RoomItem>>(cx, || Vec::new());
    let pattern = use_state(cx, String::new);
    let rooms_filtered = use_ref(cx, || Vec::new());
    let selected_space = use_ref::<String>(cx, || String::new());
    let title_header =
        use_shared_state::<TitleHeaderMain>(cx).expect("Unable to read title header");

    let on_click_room = move |evt: FormRoomEvent| {
        room.set(evt.room.clone());
        room_tabs.with_mut(|tabs| tabs.insert(evt.room, vec![]));
        messages.reset();
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
            title_header,
            session,
            notification,
            key_chat_list_home,
            key_session_error_not_found
        ];

        async move {
            let Some(session_data) = session.get() else {
                return notification.handle_error(&key_session_error_not_found);
            };

            let Conversations {
                rooms: r,
                spaces: s,
            } = list_rooms_and_spaces(&client, session_data).await;

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

            selected_space.set(key_chat_list_home.clone());
            title_header.write().title = key_chat_list_home.clone();
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
                          selected_space.set(key_chat_list_home.clone());
                          title_header.write().title = key_chat_list_home.clone();
                      },
                      Avatar {
                          name: key_chat_list_home.clone(),
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
                            placeholder: "{key_chat_list_search}",
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

            if !room.get().name.is_empty() {
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
