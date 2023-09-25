use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use log::info;
use matrix_sdk::ruma::{OwnedUserId, UserId};

use crate::{
    components::atoms::{Header, MessageInput, RoomView},
    hooks::use_client::use_client,
    services::matrix::matrix::create_room,
};
use futures_util::StreamExt;

#[derive(Clone)]
pub struct Profile {
    displayname: String,
    avatar_uri: Option<String>,
}

pub fn RoomNew(cx: Scope) -> Element {
    let navigation = use_navigator(cx);
    let client = use_client(cx);
    let user_id = use_state::<String>(cx, || String::from("@brayan-test-1:matrix.org"));
    let user = use_state::<Option<Profile>>(cx, || None);
    let error = use_state::<Option<String>>(cx, || None);

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
                    Ok(u) => user.set(Some(Profile {
                        displayname: String::from(u.displayname.unwrap()),
                        avatar_uri: None,
                    })),
                    Err(err) => {
                        info!("{err:?}");
                    }
                }
            }
        }
    });

    let on_handle_create = move |_| {
        cx.spawn({
            to_owned![client, user_id, user];

            async move {
                let u = UserId::parse(&user_id.get()).unwrap();
                let x = create_room(&client.get(), true, &[u], None).await;

                info!("{x:?}")
            }
        })
    };

    render! {
        Header {
            text: "New Chat",
            on_event: move |_|{
                navigation.go_back()
            }
        }
        rsx!(
            MessageInput{
                itype: "text",
                message: "{user_id.get()}",
                placeholder: "Escribe el id",
                error: error.get().as_ref(),
                on_input: move |event: Event<FormData>| {
                    user_id.set(event.value.clone());
                },
                on_keypress: move |_| {
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
                            avatar_uri: user.avatar_uri.as_ref(),
                            description: "Start your chat with {user.displayname}",
                            on_click: on_handle_create
                        }
                    }
                )
            }
        )
    }
}
