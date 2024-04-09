use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};
use futures::TryFutureExt;
use ruma::RoomId;

use crate::{
    components::{
        atoms::{
            button::Variant,
            header_main::{HeaderCallOptions, HeaderEvent},
            Avatar, Button, Header,
        },
        molecules::rooms::CurrentRoom,
    },
    hooks::{
        use_client::use_client,
        use_notification::use_notification,
        use_room::use_room,
        use_room_preview::{use_room_preview, PreviewRoom},
        use_rooms::use_rooms,
    },
    pages::route::Route,
    services::matrix::matrix::join_room,
};

pub enum PreviewRoomError {
    InvalidRoomId,
    InvitationNotFound,
    AcceptFailed,
    JoinFailed,
}

#[derive(PartialEq, Props, Clone)]
pub struct PreviewRoomProps {
    on_back: EventHandler<()>,
}
pub fn PreviewRoom(props: PreviewRoomProps) -> Element {
    let i18 = use_i18();
    let nav = use_navigator();
    let mut preview = use_room_preview();
    let mut room = use_room();
    let mut rooms = use_rooms();
    let client = use_client();
    let mut notification = use_notification();

    let header_event = move |evt: HeaderEvent| match evt.value {
        HeaderCallOptions::CLOSE => {
            nav.push(Route::ChatList {});
            preview.set(PreviewRoom::default());
            props.on_back.call(())
        }
        _ => {}
    };

    let mut on_handle_error = move |e: PreviewRoomError| {
        let message = match e {
            PreviewRoomError::InvalidRoomId => translate!(i18, "chat.common.error.room_id"),
            PreviewRoomError::InvitationNotFound => translate!(i18, "chat.preview_error_not_found"),
            PreviewRoomError::AcceptFailed => translate!(i18, "chat.preview_error_accept"),
            PreviewRoomError::JoinFailed => translate!(i18, "chat.preview_error_join"),
        };

        notification.handle_error(&message);
    };

    let on_handle_accept_invitation = move |r: Rc<CurrentRoom>| {
        spawn({
            async move {
                let room_id = RoomId::parse(&*r.id).map_err(|_| PreviewRoomError::InvalidRoomId)?;
                let invitation = client
                    .get()
                    .get_invited_room(&room_id)
                    .ok_or(PreviewRoomError::InvitationNotFound)?;

                invitation
                    .accept_invitation()
                    .await
                    .map_err(|_| PreviewRoomError::AcceptFailed)?;

                preview.default();
                room.set(CurrentRoom {
                    id: r.id.to_string(),
                    name: r.name.to_string(),
                    avatar_uri: r.avatar_uri.clone(),
                });

                let item = rooms
                    .remove_invited(&room_id.to_string())
                    .map_err(|_| PreviewRoomError::InvitationNotFound)?;

                rooms.push_joined(item);

                Ok::<(), PreviewRoomError>(())
            }
            .unwrap_or_else(on_handle_error)
        })
    };

    let on_handle_reject_invitation = move |r: Rc<CurrentRoom>| {
        spawn({
            async move {
                let room_id = RoomId::parse(&*r.id).map_err(|_| PreviewRoomError::InvalidRoomId)?;
                let invitation = client
                    .get()
                    .get_invited_room(&room_id)
                    .ok_or(PreviewRoomError::InvitationNotFound)?;

                invitation
                    .reject_invitation()
                    .await
                    .map_err(|_| PreviewRoomError::AcceptFailed)?;

                preview.default();
                room.default();

                rooms
                    .remove_invited(&room_id.to_string())
                    .map_err(|_| PreviewRoomError::InvitationNotFound)?;

                Ok::<(), PreviewRoomError>(())
            }
            .unwrap_or_else(on_handle_error)
        })
    };

    let on_handle_join = move |r: Rc<CurrentRoom>| {
        spawn({
            async move {
                let room_id = RoomId::parse(&*r.id).map_err(|_| PreviewRoomError::InvalidRoomId)?;

                join_room(&client.get(), &room_id)
                    .await
                    .map_err(|_| PreviewRoomError::JoinFailed)?;

                room.set(CurrentRoom {
                    id: r.id.to_string(),
                    name: r.name.to_string(),
                    avatar_uri: r.avatar_uri.clone(),
                });

                Ok::<(), PreviewRoomError>(())
            }
            .unwrap_or_else(on_handle_error)
        });
    };

    let on_handle_back = move || {
        spawn(async move {
            preview.default();
            room.default();
        });
    };

    rsx! {
        div { class: "active-room",
            match preview.get() {
                PreviewRoom::Invited(room) => {
                    let room = Rc::new(room);
                    let room_to_header = room.clone();
                    let room_to_avatar = room.clone();
                    let room_action_accept = room.clone();
                    let room_action_reject = room.clone();

                    rsx!(
                        Header {
                            text: "{room_to_header.name.clone()}",
                            avatar_element: rsx!(
                                Avatar {
                                    name: room_to_header.name.to_string(),
                                    size: 32,
                                    uri: room_to_header.avatar_uri.clone()
                                }
                            ),
                            on_event: header_event
                        }

                        section {
                            class: "preview-room",
                            h3 {
                                class: "preview-room__title",
                                {translate!(i18, "chat.preview.invited.title")} "{room.name.clone()}?"
                            }
                            Avatar {
                                name: room_to_avatar.name.to_string(),
                                size: 32,
                                uri: room_to_avatar.avatar_uri.clone()
                            }
                            div {
                                class: "preview-room__content",
                                Button {
                                    text: translate!(i18, "chat.preview.invited.cta.accept"),
                                    on_click: move |_| {
                                        on_handle_accept_invitation(room_action_accept.clone());
                                    },
                                    status: None
                                }

                                Button {
                                    text: translate!(i18, "chat.preview.invited.cta.reject"),
                                    variant: Variant::Tertiary,
                                    on_click: move |_| {
                                        on_handle_reject_invitation(room_action_reject.clone());
                                    },
                                    status: None
                                }
                            }

                        }
                    )
                }
                PreviewRoom::Joining(room) => {
                    let room = Rc::new(room);
                    let room_to_header = room.clone();
                    let room_action_join = room.clone();

                    rsx!(
                        Header {
                            text: "{room_to_header.name.clone()}",
                            avatar_element: rsx!(
                                Avatar {
                                    name: room_to_header.name.to_string(),
                                    size: 32,
                                    uri: room_to_header.avatar_uri.clone()
                                }
                            ),
                            on_event: header_event
                        }

                        section {
                            class: "preview-room",
                            h3 {
                                class: "preview-room__title",
                                {translate!(i18, "chat.preview.join.title")}
                            }
                            div {
                                class: "preview-room__content",
                                Button {
                                    text: translate!(i18, "chat.preview.join.cta.accept"),
                                    on_click: move |_| {
                                        on_handle_join(room_action_join.clone());
                                    },
                                    status: None
                                }

                                Button {
                                    text: translate!(i18, "chat.preview.join.cta.back"),
                                    variant: Variant::Tertiary,
                                    on_click: move |_| {
                                        on_handle_back()
                                    },
                                    status: None
                                }
                            }

                        }
                    )
                }
                PreviewRoom::Creating(room) => {
                    rsx!(
                        Header {
                            text: "{room.name.clone()}",
                            avatar_element: rsx!(
                                Avatar {
                                    name: room.name.to_string(),
                                    size: 32,
                                    uri: room.avatar_uri.clone()
                                }
                            ),
                            on_event: header_event
                        }
                    )
                }
                _ => None
            }
        }
    }
}
