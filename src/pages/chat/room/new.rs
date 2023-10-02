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
    hooks::use_client::use_client,
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
    let current_room = use_shared_state::<CurrentRoom>(cx).unwrap();
    let user_id = use_state::<String>(cx, || String::from("@brayan-test-1:matrix.org"));
    let user = use_state::<Option<Profile>>(cx, || None);
    let error_field = use_state::<Option<String>>(cx, || None);
    let error_creation = use_state::<Option<String>>(cx, || None);

    let task_search_user = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![client, user];

        async move {
            while let Some(id) = rx.next().await {
                let u = UserId::parse(&id).unwrap();
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
                            displayname: String::from(u.displayname.unwrap()),
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
                current_room,
                user
            ];

            async move {
                let u = UserId::parse(&user_id.get()).unwrap();
                let room_meta = create_room(&client.get(), true, &[u], None, None).await;

                info!("{room_meta:?}");

                match room_meta {
                    Ok(res) => {
                        let room_id = res.room_id.to_string();

                        let Profile {
                            displayname,
                            avatar_uri,
                        } = user.get().clone().unwrap();

                        *current_room.write() = CurrentRoom {
                            id: room_id.clone(),
                            name: displayname,
                            avatar_uri: avatar_uri,
                        };

                        info!("{:?}", *current_room.read());

                        navigation.push(Route::ChatRoom { name: room_id });
                    }
                    Err(_) => {
                        let e = Some(String::from("Ha ocurrido un error al crear el DM"));
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
                        style: r#"
                            margin-top: 10px;
                        "#,
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
