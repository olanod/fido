use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::{
        atoms::{
            header_main::{HeaderCallOptions, HeaderEvent},
            Avatar, Close, Header, Icon,
        },
        molecules::{input_message::FormMessageEvent, rooms::CurrentRoom, InputMessage, List},
    },
    hooks::{
        use_chat::{use_chat, UseChat},
        use_client::use_client,
        use_messages::use_messages,
        use_reply::use_reply,
        use_room::use_room,
        use_send_attach::use_send_attach,
        use_send_message::use_send_message,
        use_thread::use_thread,
    },
    pages::{chat::chat::MessageItem, route::Route},
    services::matrix::matrix::{Attachment, AttachmentStream},
};

#[derive(PartialEq, Props, Clone)]
pub struct ActiveRoomProps {
    on_back: EventHandler<()>,
}
pub fn ActiveRoom(props: ActiveRoomProps) -> Element {
    let i18 = use_i18();
    let nav = use_navigator();
    let mut room = use_room();
    let mut rooms = use_rooms();
    let messages = use_messages();
    let client = use_client();
    let mut notification = use_notification();
    let send_message = use_send_message();
    let send_attach = use_send_attach();

    let mut replying_to = use_reply();
    let mut threading_to = use_thread();

    let mut use_m = use_chat();
    let mut use_t = use_chat();
    let UseChat {
        messages: _,
        isLoading: is_loading,
        limit: _,
        task: _,
    } = use_m.get();

    let mut messages_lifecycle = messages.clone();
    let mut replying_to_lifecycle = replying_to.clone();
    let mut threading_to_lifecycle = threading_to.clone();
    let messages = messages.get();

    let input_placeholder =
        use_signal::<String>(|| translate!(i18, "chat.inputs.plain_message.placeholder"));

    use_drop(move || {
        messages_lifecycle.set(vec![]);
        replying_to_lifecycle.set(None);
        threading_to_lifecycle.set(None);
    });

    let header_event = move |evt: HeaderEvent| match evt.value {
        HeaderCallOptions::CLOSE => {
            nav.push(Route::ChatList {});
            room.set(CurrentRoom::default());
            props.on_back.call(())
        }
        _ => {}
    };

    let input_message_event = move |evt: HeaderEvent| match evt.value {
        HeaderCallOptions::CLOSE => {
            replying_to.set(None);
        }
        _ => {}
    };

    let mut on_push_message = move |evt: FormMessageEvent, send_to_thread: bool| {
        let reply_to = replying_to.get().map(|r| r.event_id);

        send_message.send(MessageItem {
            room_id: room.get().id.clone(),
            msg: evt.value,
            reply_to,
            send_to_thread,
        });
    };

    let on_handle_attach = move |attachment: Attachment, send_to_thread: bool| {
        send_attach.send(AttachmentStream {
            attachment,
            send_to_thread,
        });
    };

    let on_handle_leave = move |_| {
        spawn({
            async move {
                leave_room(&client.get(), &room.get().id).await?;
                rooms
                    .remove_joined(&room.get().id)
                    .map_err(|_| LeaveRoomError::RoomNotFound)?;
                room.default();

                Ok::<(), LeaveRoomError>(())
            }
            .unwrap_or_else(move |e: LeaveRoomError| {
                let message = match e {
                    LeaveRoomError::InvalidRoomId => translate!(i18, "chat.common.error.room_id"),
                    LeaveRoomError::RoomNotFound => {
                        translate!(i18, "chat.common.error.room_not_found")
                    }
                    LeaveRoomError::Failed => translate!(i18, "chat.actions.leave"),
                };

                notification.handle_error(&message);
            })
        });
    };

    let mut show_room_menu = use_signal(|| false);
    let on_handle_menu = move |_| {
        spawn(async move {
            show_room_menu.toggle();
        });
    };

    rsx! {
        div {
            class: "active-room",
            Header {
                text: "{room.get().name.clone()}",
                avatar_element: rsx!(
                    Avatar {
                        name: (room.get()).name.to_string(),
                        size: 32,
                        uri: room.get().avatar_uri.clone()
                    }
                ),
                menu: rsx!(
                    section {
                        button {
                            class: "nav__cta",
                            onclick: on_handle_menu,
                            if show_room_menu() {
                                Icon {
                                    stroke: "var(--text-1)",
                                    icon: ArrowUpCircle,
                                    height: 24,
                                    width: 24
                                }
                            } else {
                                Icon {
                                    stroke: "var(--text-1)",
                                    icon: ArrowDownCircle,
                                    height: 24,
                                    width: 24
                                }
                            },
                        }
                        if show_room_menu() {
                            div {
                                class: "room-menu",
                                ul {
                                    li {
                                        class: "room-menu__item",
                                        button {
                                            class: "room-menu__cta",
                                            onclick: on_handle_leave,
                                            Icon {
                                                stroke: "var(--text-1)",
                                                icon: Exit
                                            }
                                            span {
                                                {translate!(i18, "chat.room-menu.leave")}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                ),
                on_event: header_event
            }
            List {
                messages: messages.clone(),
                thread: None,
                is_loading: is_loading,
                show_load_button: true,
                on_scroll: move |_| {
                    use_m.loadmore("{room().id}");
                }
            },
            InputMessage {
                placeholder: input_placeholder().as_str(),
                on_submit: move |event| {
                    on_push_message(event, false);
                },
                on_event: input_message_event,
                on_attach: move |event|{
                    on_handle_attach(event, false);
                }
            }
        }

        if let Some(t) = threading_to.get() {
            div {
                class: "active-room__thread",
                // thread title
                div {
                    class: "active-room__thread__head",
                    p {
                        class: "active-room__thread__title",
                        {translate!(i18, "chat.thread.title")}
                    }
                    button {
                        class: "active-room__close",
                        onclick: move |_| {
                            threading_to.set(None)
                        },
                        Icon {
                            stroke: "var(--icon-subdued)",
                            icon: Close,
                            height: 24,
                            width: 24
                        }
                    }
                }

                // thread messages
                List {
                    messages: vec![],
                    thread: Some(t.thread.clone()),
                    is_loading: is_loading,
                    on_scroll: move |_| {
                        use_t.loadmore("{room.get().id}");
                    }
                },
                InputMessage {
                    placeholder: input_placeholder().as_str(),
                    on_submit: move |event| {
                        on_push_message(event, true);
                    },
                    on_event: input_message_event,
                    on_attach: move |event|{
                        on_handle_attach(event, true);
                    }
                }
            }
        }
    }
}
