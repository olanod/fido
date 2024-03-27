use std::collections::HashMap;

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
        use_auth::use_auth, use_client::use_client, use_init_app::BeforeSession, use_notification::use_notification, use_room::use_room, use_session::use_session
    },
    pages::chat::room::group::{CreateRoomError, Profile},
    services::matrix::matrix::{create_room, find_user_by_id},
    utils::{i18n_get_key_value::i18n_get_key_value, sync_room::sync_created_room},
};
use futures_util::{StreamExt, TryFutureExt};

pub enum CreationStatus {
    Start,
    Creating,
    Ok,
    Error(CreateRoomError),
}

pub fn RoomNew(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    let key_common_error_user_id = translate!(i18, "chat.common.error.user_id");
    let key_common_error_server = translate!(i18, "chat.common.error.server");

    let key_dm_error_not_found = translate!(i18, "dm.error.not_found");
    let key_dm_error_profile = translate!(i18, "dm.error.profile");

    let key_chat_guest_signup_description = translate!(i18, "chat.guest.signup.description");
    let key_chat_guest_signup_cta = translate!(i18, "chat.guest.signup.cta");

    let key_dm_title = "dm-title";
    let key_dm_label = "dm-label";
    let key_dm_placeholder = "dm-placeholder";
    let key_dm_description = "dm-description";

    let i18n_map = HashMap::from([
        (key_dm_title, translate!(i18, "dm.title")),
        (key_dm_label, translate!(i18, "dm.label")),
        (key_dm_placeholder, translate!(i18, "dm.placeholder")),
        (key_dm_description, translate!(i18, "dm.description")),
    ]);

    let navigation = use_navigator(cx);
    let client = use_client(cx);
    let notification = use_notification(cx);
    let room = use_room(cx);
    let session = use_session(cx);
    let auth = use_auth(cx);

    let before_session =
        use_shared_state::<BeforeSession>(cx).expect("Unable to use before session");

    let user_id = use_state::<String>(cx, || String::from("@edith-test-1:matrix.org"));
    let user = use_state::<Option<Profile>>(cx, || None);
    let error_field = use_state::<Option<String>>(cx, || None);
    let status = use_state::<CreationStatus>(cx, || CreationStatus::Start);

    let task_search_user = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![client, user, notification, key_dm_error_not_found];

        async move {
            while let Some(id) = rx.next().await {
                match find_user_by_id(&id, &client.get()).await {
                    Ok(profile) => user.set(Some(profile)),
                    Err(_) => {
                        notification.handle_error(&key_dm_error_not_found);
                    }
                }
            }
        }
    });

    let on_handle_create = move |_| {
        if session.is_guest() {
            return;
        }

        cx.spawn({
            to_owned![
                client,
                user_id,
                navigation,
                room,
                user,
                key_common_error_user_id,
                key_dm_error_profile,
                key_dm_error_not_found,
                key_common_error_server,
                notification,
                status
            ];

            let status_error = status.clone();

            async move {
                status.set(CreationStatus::Creating);
                let u =
                    UserId::parse(&user_id.get()).map_err(|_| CreateRoomError::InvalidUserId)?;

                let room_meta = create_room(&client.get(), true, &[u], None, None)
                    .await
                    .map_err(|_| CreateRoomError::ServerError)?;

                let room_id = room_meta.room_id.to_string();

                let profile = user.get().clone().ok_or(CreateRoomError::InvalidUserId)?;

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
                    CreateRoomError::InvalidUserId => &key_common_error_user_id,
                    CreateRoomError::UserNotFound => &key_dm_error_profile,
                    CreateRoomError::InvalidUsername => &key_dm_error_not_found,
                    CreateRoomError::ServerError => &key_common_error_server,
                };

                status_error.set(CreationStatus::Error(e.clone()));
                notification.handle_error(&message_error);
            })
        })
    };

    render! {
        Header {
            text: "{i18n_get_key_value(&i18n_map, key_dm_title)}",
            on_event: move |_|{
                navigation.go_back()
            }
        }
        rsx!(
            MessageInput{
                message: "{user_id.get()}",
                placeholder: "{i18n_get_key_value(&i18n_map, key_dm_placeholder)}",
                label: "{i18n_get_key_value(&i18n_map, key_dm_label)}",
                error: error_field.get().as_ref(),
                on_input: move |event: Event<FormData>| {
                    user_id.set(event.value.clone());
                },
                on_keypress: move |event: KeyboardEvent| {
                    if event.code() == keyboard_types::Code::Enter && !user_id.get().is_empty() {
                        let id = user_id.get();
                        task_search_user.send(id.to_string())
                    }
                },
                on_click: move |_| {
                    let id = user_id.get();
                    task_search_user.send(id.to_string())
                },
            }
            if let Some(user) = user.get() {
                let on_handle_create = on_handle_create.clone();
                rsx!(
                    div {
                        class: "room-new__items",
                        RoomView {
                            displayname: "{user.displayname}",
                            avatar_uri: user.avatar_uri.clone(),
                            description: "{i18n_get_key_value(&i18n_map, key_dm_description)} {user.displayname}",
                            on_click: on_handle_create
                        }
                    }
                )
            }
            if session.is_guest() {
                rsx!(
                    Guest {
                        description: "{key_chat_guest_signup_description}",
                        cta: "{key_chat_guest_signup_cta}",
                        on_click: move |_| {
                            auth.set_logged_in(false);
                            *before_session.write() = BeforeSession::Signup;
                        }
                    }
                )
            }
            match status.get() {
                CreationStatus::Creating => {
                    render!(rsx! {
                        div {
                            class: "room-new__status-container",
                            p {
                                class: "room-new__status__description",
                                translate!(i18, "dm.status.creating")
                            }
                        }
                    })
                },
                CreationStatus::Ok => {
                    render!(rsx! {
                        div {
                            class: "room-new__status-container",
                            p {
                                class: "room-new__status__description",
                                translate!(i18, "dm.status.created")
                            }
                        }
                    })
                },
                CreationStatus::Error(CreateRoomError::ServerError) => {
                    let cta_back = translate!(i18, "dm.status.error.cta.back");
                    let cta_try = translate!(i18, "dm.status.error.cta.try");
                    render!(rsx! {
                        div {
                            class: "room-new__status-container",
                            h3 {
                                class: "room-new__status__title",
                                translate!(i18, "dm.status.error.title")
                            }
                            p {
                                class: "room-new__status__description",
                                translate!(i18, "dm.status.error.description")
                            }
                            div {
                                class: "row room-new__status-cta",
                                Button{
                                    text: "{cta_back}",
                                    variant: &Variant::Secondary,
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
                    })
                },
                _ => None
            }
        )
    }
}
