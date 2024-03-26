use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::{
        atoms::{
            helper::HelperData, input::InputType, message::Messages, room::RoomItem, Helper,
            MessageInput, Space, SpaceSkeleton,
        },
        molecules::{
            rooms::{CurrentRoom, FormRoomEvent},
            RoomsList,
        },
        organisms::{
            chat::{
                utils::handle_command::{self, Command},
                ActiveRoom, PreviewRoom, PublicRooms,
            },
            main::TitleHeaderMain,
        },
    },
    hooks::{
        use_client::use_client,
        use_lifecycle::use_lifecycle,
        use_messages::use_messages,
        use_notification::use_notification,
        use_public::{use_public, PublicState},
        use_room::use_room,
        use_room_preview::{use_room_preview, PreviewRoom},
        use_rooms::{use_rooms, RoomsList},
        use_session::use_session,
    },
    pages::chat::chat::MessageItem,
    services::matrix::matrix::{
        invited_rooms, list_rooms_and_spaces, public_rooms_and_spaces, Conversations,
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
    let is_loading = use_state(cx, || false);

    let r = room.clone();
    use_lifecycle(
        &cx,
        || {},
        move || {
            to_owned![r];

            r.default();
        },
    );

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
            key_session_error_not_found,
            is_loading
        ];

        async move {
            is_loading.set(true);
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

            is_loading.set(false);
        }
    });

    enum ScrollToPosition {
        Top,
        Bottom,
        Right,
        Left,
        Custom(f64, f64),
    }

    let on_scroll_chat_list_wrapper = move |position: ScrollToPosition| {
        if let Some(e) = chat_list_wrapper_ref.read().as_ref() {
            let (x, y) = match position {
                ScrollToPosition::Top | ScrollToPosition::Left => (0.0, 0.0),
                ScrollToPosition::Bottom => (0.0, e.get_bounding_client_rect().height()),
                ScrollToPosition::Right => (e.get_bounding_client_rect().width(), 0.0),
                ScrollToPosition::Custom(x, y) => (x, y),
            };
            e.scroll_to_with_x_and_y(x, y);
        }
    };

    let on_click_invitation = move |evt: FormRoomEvent| {
        preview.set(PreviewRoom::Invited(evt.room.clone()));
        room.default();
    };

    let on_click_room = move |evt: FormRoomEvent| {
        room.set(evt.room.clone());
        room_tabs.with_mut(|tabs| tabs.insert(evt.room, vec![]));
        messages.reset();
        preview.default();

        on_scroll_chat_list_wrapper(ScrollToPosition::Right);
    };

    let on_click_helper = move |_| {
        on_scroll_chat_list_wrapper(ScrollToPosition::Right);
        cx.spawn({
            to_owned![client, notification, public];
            async move {
                let message_item = MessageItem {
                    room_id: String::new(),
                    msg: String::from("!rooms"),
                    reply_to: None,
                    send_to_thread: false,
                };
                match handle_command::handle_command(&message_item, &client).await {
                    Ok(Command::Join(_)) => {}
                    Ok(Command::PublicRooms) => public.set(PublicState { show: true }),
                    Err(error) => {
                        let message = match error {
                            _ => "Error",
                        };

                        notification.handle_error(message);
                    }
                }
            }
        })
    };

    render! {
        section {
            class: "chat-list options",
            div {
                if !spaces.get().is_empty() {
                    rsx!(
                        ul {
                            class: "chat-list__wrapper",
                            Space {
                                text: "{key_chat_list_home}",
                                uri: None,
                                on_click: move |_| {
                                    rooms_to_list.set(rooms.get().clone());
                                    rooms_filtered.set(rooms.get().clone());
                                    selected_space.set(key_chat_list_home.clone());
                                    title_header.write().title = key_chat_list_home.clone();

                                    if !rooms.get().iter().any(|r| {
                                        room.get().id.eq(&r.id)
                                    }) {
                                        room.default()
                                    }
                                }
                            }

                            spaces.get().iter().map(|(space, value)|{
                                let name = space.name.clone();
                                rsx!(
                                    Space {
                                        text: "{name}",
                                        uri: space.avatar_uri.clone(),
                                        on_click: move |_| {
                                            rooms_to_list.set(value.clone());
                                            rooms_filtered.set(value.clone());
                                            selected_space.set(space.name.clone());
                                            title_header.write().title = space.name.clone();

                                            if !value.iter().any(|r| {
                                                room.get().id.eq(&r.id)
                                            }) {
                                                room.default()
                                            }
                                        }
                                    }
                                )
                            })
                        }
                    )
                } else if *is_loading.get() {
                    rsx!(
                        ul {
                            class: "chat-list__wrapper",
                            (0..5).map(|_| {
                                rsx!(
                                    SpaceSkeleton {
                                        size: 50
                                    }
                                )
                            })
                        }
                    )
                } else {
                    rsx!( div {})
                }
            }

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

                            if !event.value.is_empty() {
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
                        is_loading: *is_loading.get(),
                        on_submit: on_click_room
                    }
                }
            )


            div {
                class: "chat-list__content",
                onclick: move |_| {
                    on_scroll_chat_list_wrapper(ScrollToPosition::Right)
                },
                if public.get().show {
                    rsx!(
                        section {
                            class: "chat-list__active-room",
                            PublicRooms {
                                on_back: move |_| {
                                    on_scroll_chat_list_wrapper(ScrollToPosition::Left)
                                }
                            }
                        }
                    )
                } else if !preview.get().is_none() {
                    rsx!(
                        section {
                            class: "chat-list__active-room",
                            PreviewRoom {
                                on_back: move |_| {
                                    on_scroll_chat_list_wrapper(ScrollToPosition::Left)
                                }
                            }
                        }
                    )
                } else if !room.get().name.is_empty(){
                    rsx!(
                        section {
                            class: "chat-list__active-room",
                            ActiveRoom {
                                on_back: move |_| {
                                    on_scroll_chat_list_wrapper(ScrollToPosition::Left)
                                }
                            }
                        }
                    )
                } else {
                    rsx!(
                        section {
                            class: "chat-list__static",
                            Helper {
                                helper: HelperData {
                                    title: key_chat_helper_rooms_title,
                                    description: key_chat_helper_rooms_description,
                                    subtitle: key_chat_helper_rooms_subtitle,
                                    example: String::from("!rooms")
                                },
                                on_click: on_click_helper
                            }
                        }
                    )
                }
            }
        }
    }
}
