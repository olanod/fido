use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures::TryFutureExt;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

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
};

pub enum ChatListError {
    SessionNotFound,
    InvitedRooms,
    PublicRooms,
}

#[component]
pub fn ChatList() -> Element {
    let i18 = use_i18();
    let client = use_client();
    let session = use_session();
    let mut notification = use_notification();
    let mut room = use_room();
    let mut public = use_public();
    let mut rooms_list = use_rooms();
    let mut preview = use_room_preview();
    let mut messages = use_messages();

    let mut room_tabs = use_signal::<HashMap<CurrentRoom, Messages>>(|| HashMap::new());

    let mut rooms = use_signal::<Vec<RoomItem>>(|| Vec::new());
    let mut all_rooms = use_signal::<Vec<RoomItem>>(|| Vec::new());
    let mut spaces = use_signal::<HashMap<RoomItem, Vec<RoomItem>>>(|| HashMap::new());
    let mut pattern = use_signal(|| String::new());
    let mut rooms_filtered = use_signal(|| Vec::new());
    let mut selected_space = use_signal::<String>(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut chat_list_wrapper_ref = use_signal::<Option<Box<HtmlElement>>>(|| None);

    let mut title_header = consume_context::<Signal<TitleHeaderMain>>();

    let mut room_lifecycle = room.clone();

    use_drop(move || room_lifecycle.default());

    use_coroutine(|_: UnboundedReceiver<()>| {
        async move {
            is_loading.set(true);

            let session_data = session.get().ok_or(ChatListError::SessionNotFound)?;

            let invited = invited_rooms(&client.get())
                .await
                .map_err(|_| ChatListError::InvitedRooms)?;

            let Conversations {
                rooms: r,
                spaces: s,
            } = list_rooms_and_spaces(&client.get(), session_data).await;

            let public_rooms = public_rooms_and_spaces(&client.get(), None, None, None)
                .await
                .map_err(|_| ChatListError::PublicRooms)?;

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

            rooms_list.set(RoomsList {
                public: public_rooms.rooms,
                invited,
                joined: r.clone(),
            });
            rooms_filtered.set(r);

            selected_space.set(translate!(i18, "chat.list.home"));
            title_header.write().title = translate!(i18, "chat.list.home");

            is_loading.set(false);

            Ok::<(), ChatListError>(())
        }
        .unwrap_or_else(move |e: ChatListError| {
            let message = match e {
                ChatListError::SessionNotFound => translate!(i18, "chat.session.error.not_found"),
                ChatListError::PublicRooms => translate!(i18, "chat.list.errors.public_rooms"),
                ChatListError::InvitedRooms => translate!(i18, "chat.list.errors.invited_rooms"),
            };

            notification.handle_error(&message);
        })
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
        spawn({
            async move {
                let message_item = MessageItem {
                    room_id: String::new(),
                    msg: String::from("!rooms"),
                    reply_to: None,
                    send_to_thread: false,
                };
                match handle_command::handle_command(&message_item, &client.get()).await {
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
        });
    };

    rsx! {
        div {
            class: "chat-list-wrapper",
            onmounted: move |event| {
                event
                    .data
                    .downcast::<web_sys::Element>()
                    .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                    .map(|html_element| {
                        chat_list_wrapper_ref.set(Some(Box::new(html_element.clone())))
                    });
            },
            section { class: "chat-list options",
                div { class: "chat-list__spaces",
                    if !spaces().is_empty() {
                        ul { class: "chat-list__wrapper",
                            Space {
                                text: translate!(i18, "chat.list.home"),
                                uri: None,
                                on_click: move |_| {
                                    rooms_list.set_joined(rooms().clone());
                                    rooms_filtered.set(rooms().clone());
                                    selected_space.set(translate!(i18, "chat.list.home"));
                                    title_header.write().title = translate!(i18, "chat.list.home");
                                    if !rooms().iter().any(|r| { room.get().id.eq(&r.id) }) {
                                        room.default()
                                    }
                                }
                            }

                            for (space , value) in spaces.read().clone().into_iter() {
                                Space {
                                    text: "{space.name}",
                                    uri: space.avatar_uri.clone(),
                                    on_click: move |_| {
                                        rooms_list.set_joined(value.clone());
                                        rooms_filtered.set(value.clone());
                                        selected_space.set(space.name.clone());
                                        title_header.write().title = space.name.clone();
                                        if !value.iter().any(|r| { room.get().id.eq(&r.id) }) {
                                            room.default()
                                        }
                                    }
                                }
                            }
                        }
                    } else if is_loading() {
                        ul { class: "chat-list__wrapper",
                            {(0..5).map(|_| {
                                rsx!(
                                    SpaceSkeleton {
                                        size: 50
                                    }
                                )
                            })}
                        }
                    } else {
                        div {}
                    }
                }

                div {
                    class: "chat-list__rooms",
                    onclick: move |_| { on_scroll_chat_list_wrapper(ScrollToPosition::Left) },
                    MessageInput {
                        message: "{pattern}",
                        placeholder: translate!(i18, "chat.list.search"),
                        itype: InputType::Search,
                        error: None,
                        on_input: move |event: FormEvent| {
                            pattern.set(event.value().clone());
                            if !event.value().is_empty() {
                                let filter = all_rooms()
                                    .iter()
                                    .filter(|r| {
                                        r.name.to_lowercase().contains(&event.value().to_lowercase())
                                    })
                                    .cloned()
                                    .collect::<Vec<_>>();
                                rooms_filtered.set(filter);
                            } else {
                                rooms_filtered.set(rooms_list.get_joined())
                            }
                        },
                        on_keypress: move |_| {},
                        on_click: move |_| { on_scroll_chat_list_wrapper(ScrollToPosition::Right) }
                    }
                    div {
                        class: "chat-list__rooms__content",
                        if !rooms_list.get_invited().is_empty() {
                           div {
                                class: "chat-list__item",
                                h2 { class: "header__title header__title--sticky", {translate!(i18, "chat.list.invitate")} }
                                RoomsList {
                                    rooms: rooms_list.get_invited(),
                                    is_loading: is_loading(),
                                    on_submit: on_click_invitation
                                }
                           }
                        }

                        div {
                            class: "chat-list__item",
                            h2 { class: "header__title header__title--sticky", {translate!(i18, "chat.list.rooms")} }
                            RoomsList { rooms: rooms_filtered(), is_loading: is_loading(), on_submit: on_click_room }
                        }
                    }
                }
                div {
                    class: "chat-list__content",
                    onclick: move |_| { on_scroll_chat_list_wrapper(ScrollToPosition::Right) },
                    if public.get().show {
                        section { class: "chat-list__active-room",
                            PublicRooms { on_back: move |_| { on_scroll_chat_list_wrapper(ScrollToPosition::Left) } }
                        }
                    } else if !preview.get().is_none() {
                        section { class: "chat-list__active-room",
                            PreviewRoom { on_back: move |_| { on_scroll_chat_list_wrapper(ScrollToPosition::Left) } }
                        }
                    } else if !room.get().name.is_empty() {
                        section { class: "chat-list__active-room",
                            ActiveRoom { on_back: move |_| { on_scroll_chat_list_wrapper(ScrollToPosition::Left) } }
                        }
                    } else {
                        section { class: "chat-list__static",
                            Helper {
                                helper: HelperData {
                                    title: translate!(i18, "chat.helpers.rooms.title"),
                                    description: translate!(i18, "chat.helpers.rooms.description"),
                                    subtitle: translate!(i18, "chat.helpers.rooms.subtitle"),
                                    example: String::from("!rooms"),
                                },
                                on_click: on_click_helper
                            }
                        }
                    }
                }
            }
        }
    }
}
