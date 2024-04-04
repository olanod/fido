use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};
use matrix_sdk::ruma::UserId;

use crate::{
    components::{
        atoms::{button::Variant, Button, Header, MessageInput, RoomView},
        molecules::{rooms::CurrentRoom, Guest},
    },
    hooks::{
        use_auth::use_auth, use_client::use_client, use_init_app::BeforeSession,
        use_notification::use_notification, use_room::use_room, use_session::use_session,
    },
    pages::chat::room::group::{CreateRoomError, Profile},
    services::matrix::matrix::{create_room, find_user_by_id},
    utils::sync_room::sync_created_room,
};
use futures_util::{StreamExt, TryFutureExt};

pub enum CreationStatus {
    Start,
    Creating,
    Ok,
    Error(CreateRoomError),
}

pub fn RoomNew() -> Element {
    let i18 = use_i18();

    let navigation = use_navigator();
    let client = use_client();
    let mut notification = use_notification();
    let mut room = use_room();
    let session = use_session();
    let mut auth = use_auth();

    let mut before_session = consume_context::<Signal<BeforeSession>>();

    let mut user_id = use_signal::<String>(|| String::from(""));
    let mut user = use_signal::<Option<Profile>>(|| None);
    let error_field = use_signal::<Option<String>>(|| None);
    let mut status = use_signal::<CreationStatus>(|| CreationStatus::Start);

    let task_search_user = use_coroutine(|mut rx: UnboundedReceiver<String>| async move {
        while let Some(id) = rx.next().await {
            match find_user_by_id(&id, &client.get()).await {
                Ok(profile) => user.set(Some(profile)),
                Err(_) => {
                    notification.handle_error(&translate!(i18, "chat.common.error.user_id"));
                }
            }
        }
    });

    let on_handle_create = move |_| {
        if session.is_guest() {
            return;
        }

        spawn({
            async move {
                status.set(CreationStatus::Creating);
                let u = UserId::parse(&user_id()).map_err(|_| CreateRoomError::InvalidUserId)?;

                let room_meta = create_room(&client.get(), true, &[u], None, None)
                    .await
                    .map_err(|_| CreateRoomError::ServerError)?;

                let room_id = room_meta.room_id.to_string();

                let profile = user().clone().ok_or(CreateRoomError::InvalidUserId)?;

                status.set(CreationStatus::Ok);

                sync_created_room(&room_meta.room_id, &client.get()).await;

                room.set(CurrentRoom {
                    id: room_id.clone(),
                    name: profile.displayname,
                    avatar_uri: profile.avatar_uri,
                });

                navigation.go_back();

                Ok::<(), CreateRoomError>(())
            }
            .unwrap_or_else(move |e: CreateRoomError| {
                let message_error = match e {
                    CreateRoomError::InvalidUserId => translate!(i18, "chat.common.error.user_id"),
                    CreateRoomError::UserNotFound => translate!(i18, "chat.common.error.user_id"),
                    CreateRoomError::InvalidUsername => {
                        translate!(i18, "chat.common.error.user_id")
                    }
                    CreateRoomError::ServerError => translate!(i18, "chat.common.error.user_id"),
                };

                status.set(CreationStatus::Error(e.clone()));
                notification.handle_error(&message_error);
            })
        });
    };

    rsx! {
        Header {
            text: translate!(i18, "dm.title"),
            on_event: move |_|{
                navigation.go_back()
            }
        }

            MessageInput{
                message: "{user_id()}",
                placeholder: translate!(i18, "dm.placeholder"),
                label: translate!(i18, "dm.label"),
                error: error_field(),
                on_input: move |event: Event<FormData>| {
                    user_id.set(event.value().clone());
                },
                on_keypress: move |event: KeyboardEvent| {
                    if event.code() == keyboard_types::Code::Enter && !user_id().is_empty() {
                        task_search_user.send(user_id())
                    }
                },
                on_click: move |_| {
                    task_search_user.send(user_id())
                },
            }
            if let Some(user) = user() {
                {let on_handle_create = on_handle_create.clone();
                rsx!(
                    div {
                        class: "room-new__items",
                        RoomView {
                            displayname: "{user.displayname}",
                            avatar_uri: user.avatar_uri.clone(),
                            description: r#"{translate!(i18, "dm.description")} {user.displayname}"#,
                            on_click: on_handle_create
                        }
                    }
                )}
            }
            if session.is_guest() {
                Guest {
                    description: translate!(i18, "chat.guest.signup.description"),
                    cta: translate!(i18, "chat.guest.signup.cta"),
                    on_click: move |_| {
                        auth.set_logged_in(false);
                        *before_session.write() = BeforeSession::Signup;
                    }
                }
            }
            match *status.read() {
                CreationStatus::Creating => {
                    rsx! {
                        div {
                            class: "room-new__status-container",
                            p {
                                class: "room-new__status__description",
                                {translate!(i18, "dm.status.creating")}
                            }
                        }
                    }
                },
                CreationStatus::Ok => {
                    rsx! {
                        div {
                            class: "room-new__status-container",
                            p {
                                class: "room-new__status__description",
                                {translate!(i18, "dm.status.created")}
                            }
                        }
                    }
                },
                CreationStatus::Error(CreateRoomError::ServerError) => {
                    let cta_back = translate!(i18, "dm.status.error.cta.back");
                    let cta_try = translate!(i18, "dm.status.error.cta.try");
                    rsx! {
                        div {
                            class: "room-new__status-container",
                            h3 {
                                class: "room-new__status__title",
                                {translate!(i18, "dm.status.error.title")}
                            }
                            p {
                                class: "room-new__status__description",
                                {translate!(i18, "dm.status.error.description")}
                            }
                            div {
                                class: "row room-new__status-cta",
                                Button{
                                    text: "{cta_back}",
                                    variant: Variant::Secondary,
                                    on_click: move |_| {
                                        navigation.go_back()
                                    },
                                    status: None
                                }
                                Button{
                                    text: "{cta_try}",
                                    on_click: on_handle_create,
                                    status: None
                                }
                            }
                        }
                    }
                },
                _ => None
            }

    }
}
