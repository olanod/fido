use std::{collections::HashMap, ops::Deref};

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};
use log::info;
use matrix_sdk::ruma::UserId;

use crate::{
    components::{
        atoms::{Header, MessageInput, RoomView},
        molecules::rooms::CurrentRoom,
    },
    hooks::{use_client::use_client, use_notification::use_notification, use_room::use_room},
    pages::route::Route,
    services::matrix::matrix::create_room,
    utils::i18n_get_key_value::i18n_get_key_value,
    utils::matrix::{mxc_to_https_uri, ImageSize},
};
use futures_util::StreamExt;

#[derive(Clone)]
pub struct Profile {
    displayname: String,
    avatar_uri: Option<String>,
}

pub fn RoomNew(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    let key_common_error_thread_id = translate!(i18, "chat.common.error.thread_id");
    let key_common_error_event_id = translate!(i18, "chat.common.error.event_id");
    let key_common_error_room_id = translate!(i18, "chat.common.error.room_id");
    let key_dm_error_not_found = translate!(i18, "chat.common.error.user_id");

    let key_dm_error_not_found = translate!(i18, "dm.error.not_found");
    let key_dm_error_dm = translate!(i18, "dm.error.dm");
    let key_dm_error_profile = translate!(i18, "dm.error.profile");
    let key_dm_error_file = translate!(i18, "dm.error.file");

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

    let user_id = use_state::<String>(cx, || String::from(""));
    let user = use_state::<Option<Profile>>(cx, || None);
    let error_field = use_state::<Option<String>>(cx, || None);
    let error_creation = use_state::<Option<String>>(cx, || None);

    let task_search_user = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![client, user];

        async move {
            while let Some(id) = rx.next().await {
<<<<<<< HEAD
                let u = UserId::parse(&id).unwrap();
=======
                let u = match UserId::parse(&id) {
                    Ok(u) => u,
                    Err(_) => {
                        notification.handle_error("{key_common_error_user_id}");
                        return;
                    }
                };
>>>>>>> 190ae6f (ref(i18n): complete translations)
                let u = u.deref();

                let request =
                    matrix_sdk::ruma::api::client::profile::get_profile::v3::Request::new(u);
                let resp = client.get().send(request, None).await;

                match resp {
                    Ok(u) => {
                        let avatar_uri: Option<String> = if let Some(uri) = u.avatar_url {
                            mxc_to_https_uri(
                                &uri,
                                ImageSize {
                                    width: 48,
                                    height: 48,
                                },
                            )
                        } else {
                            None
                        };

                        user.set(Some(Profile {
<<<<<<< HEAD
                            displayname: String::from(u.displayname.unwrap()),
=======
                            displayname: match u.displayname {
                                Some(d) => String::from(d),
                                None => {
                                    notification.handle_error("{key_dm_error_not_found}");
                                    return;
                                }
                            },
>>>>>>> 190ae6f (ref(i18n): complete translations)
                            avatar_uri: avatar_uri,
                        }))
                    }
                    Err(err) => {
                        info!("{err:?}");
                    }
                }
            }
        }
    });

    let on_handle_create = move |_| {
        cx.spawn({
            to_owned![
                client,
                user_id,
                error_creation,
                navigation,
                room,
                user,
                notification
            ];

            async move {
<<<<<<< HEAD
                let u = UserId::parse(&user_id.get()).unwrap();
=======
                let u = match UserId::parse(&user_id.get()) {
                    Ok(u) => u,
                    Err(_) => {
                        notification.handle_error("{key_dm_error_not_found}");
                        return;
                    }
                };
>>>>>>> 190ae6f (ref(i18n): complete translations)
                let room_meta = create_room(&client.get(), true, &[u], None, None).await;

                info!("{room_meta:?}");

                match room_meta {
                    Ok(res) => {
                        let room_id = res.room_id.to_string();

                        let Profile {
                            displayname,
                            avatar_uri,
<<<<<<< HEAD
                        } = user.get().clone().unwrap();
=======
                        } = match user.get() {
                            Some(u) => u.clone(),
                            None => {
                                notification.handle_error("{key_dm_error_not_found}");
                                return;
                            }
                        };
>>>>>>> 190ae6f (ref(i18n): complete translations)

                        room.set(CurrentRoom {
                            id: room_id.clone(),
                            name: displayname,
                            avatar_uri: avatar_uri,
                        });

                        info!("{:?}", room.get());

                        navigation.push(Route::ChatRoom { name: room_id });
                    }
                    Err(_) => {
                        let e = Some(String::from("{key_dm_error_dm}"));
                        error_creation.set(e)
                    }
                }
            }
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
                    if event.code() == keyboard_types::Code::Enter && user_id.get().len() > 0 {
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
        )
    }
}
