use std::collections::HashMap;

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{translate, i18n::use_i18};
use gloo::storage::{LocalStorage, errors::StorageError};
use log::info;
use matrix_sdk::{Client, config::SyncSettings};

use crate::{
    components::{
        atoms::MessageInput,
        organisms::{login_form::FormLoginEvent, LoginForm},
    },
    utils::i18n_get_key_value::i18n_get_key_value, services::matrix::matrix::{login, FullSession},
};

#[derive(Debug, Clone)]
pub struct LoginInfo {
    homeserver: String,
    username: String,
    password: String,
}

pub struct LoggedIn {
    pub is_logged_in: bool,
}

pub fn Login(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    let i18n_map = HashMap::from([
        ("actors-bot", translate!(i18, "login.actors.bot")),
        ("actors-user", translate!(i18, "login.actors.user")),
        (
            "homeserver-message",
            translate!(i18, "login.chat_steps.homeserver.message"),
        ),
        (
            "homeserver-placeholder",
            translate!(i18, "login.chat_steps.homeserver.placeholder"),
        ),
        (
            "username-message",
            translate!(i18, "login.chat_steps.username.message"),
        ),
        (
            "username-placeholder",
            translate!(i18, "login.chat_steps.username.placeholder"),
        ),
        (
            "password-message",
            translate!(i18, "login.chat_steps.password.message"),
        ),
        (
            "password-placeholder",
            translate!(i18, "login.chat_steps.password.placeholder"),
        ),
        (
            "messages-validating",
            translate!(i18, "login.chat_steps.messages.validating"),
        ),
        (
            "messages-welcome",
            translate!(i18, "login.chat_steps.messages.welcome"),
        ),
        (
            "invalid-url",
            translate!(i18, "login.chat_errors.invalid_url"),
        ),
        ("unknown", translate!(i18, "login.chat_errors.unknown")),
        (
            "invalid_username_password",
            translate!(i18, "login.chat_errors.invalid_username_password"),
        ),
    ]);

    let homeserver = use_state(cx, || String::from(""));
    let username = use_state(cx, || String::from(""));
    let password = use_state(cx, || String::from(""));
    let error = use_state(cx, || None);

    let logged_in = use_shared_state::<LoggedIn>(cx).unwrap();

    let login_info = use_ref::<LoginInfo>(cx, || LoginInfo {
        homeserver: String::from(""),
        username: String::from(""),
        password: String::from(""),
    });

    let on_update_homeserver = move || {
        cx.spawn({
            to_owned![login_info, homeserver, error];

            async move {
                let response = Client::builder()
                    .homeserver_url(&homeserver.get().clone())
                    .build()
                    .await;

                match response {
                    Ok(client) => {
                        login_info.with_mut(|info| info.homeserver = homeserver.get().clone());
                        info!("client: {client:?}");
                        error.set(None);
                    }
                    Err(e) => {
                        info!("error: {e:?}");
                        error.set(Some(e.to_string()));
                    }
                }
            }
        })
    };

    let on_handle_login = move || {
        login_info.with_mut(|info| info.username = username.get().clone());
        login_info.with_mut(|info| info.password = password.get().clone());

        cx.spawn({
            to_owned![logged_in, login_info, i18n_map, username, password, error];

            info!("{:?}",login_info.read());

            async move {
                let response = login(
                    login_info.read().homeserver.clone(),
                    login_info.read().username.clone(),
                    login_info.read().password.clone(),
                )
                .await;
        
                match response {
                    Ok((client, serialized_session)) => {
                        let x = <LocalStorage as gloo::storage::Storage>::set(
                            "session_file",
                            serialized_session,
                        );
        
                        info!("Session persisted in {:?}", x);
        
                        let x = sync(client.clone(), None).await;
        
                        info!("new session {:?}", x);
        
                        logged_in.write().is_logged_in = true;
                    }
                    Err(err) => {
                        info!("{:?}", err.to_string());
                        if err
                            .to_string()
                            .eq("the server returned an error: [403 / M_FORBIDDEN] Invalid username or password")
                        {
                            error.set(Some(i18n_get_key_value(
                                &i18n_map,
                                "invalid_username_password",
                            )))
                        } else {
                            error.set(Some(i18n_get_key_value(
                                &i18n_map, "unknown",
                            )))
                        }
        
                        username.set(String::from(""));
                        password.set(String::from(""));
        
                        login_info.set(LoginInfo {
                            homeserver: String::from(""),
                            username: String::from(""),
                            password: String::from(""),
                        })
                    }
                }
            }
        })
    };

    render!(if login_info.read().homeserver.len() == 0 {
        rsx!(LoginForm {
            title: "Pick a homerserver",
            description: "Join a server, by default we use https://matrix.org",
            button_text: "Confirm server",
            emoji: "🛰️",
            on_handle: move |_| { on_update_homeserver() },
            body: render!(rsx!(
                div {
                    MessageInput {
                        itype: "text",
                        message: "{homeserver.get()}",
                        placeholder: "https://matrix.org",
                        error: if homeserver.get().len() > 0 {error.get().as_ref()}else {None},
                        on_input: move |event: FormEvent| {
                            homeserver.set(event.value.clone())
                        },
                        on_keypress: move |event: KeyboardEvent| {
                            info!("{:?}", event.code());
                            if event.code() == keyboard_types::Code::Enter && homeserver.get().len() > 0 {
                                on_update_homeserver()
                            }
                        },
                        on_click: move |_| {
                            on_update_homeserver()
                        }
                    }
                }
            ))
        })
    } else if login_info.read().username.len() == 0 || login_info.read().password.len() == 0 {
        rsx!(LoginForm {
            title: "Complete your info",
            description: "Make your world, build your future",
            button_text: "Unlock app",
            emoji: "👋",
            on_handle: move |event: FormLoginEvent| {
                on_handle_login()
            },
            body: render!(rsx!(
                div {
                    MessageInput {
                        itype: "text",
                        message: "{username.get()}",
                        placeholder: "username",
                        error: if username.get().len() > 0 {error.get().as_ref()}else {None},
                        on_input: move |event: FormEvent| {
                            info!("alksdjf");
                            username.set(event.value.clone())
                        },
                        on_keypress: move |event: KeyboardEvent| {
                            if event.code() == keyboard_types::Code::Enter && username.get().len() > 0 {
                                login_info.with_mut(|info| info.username = username.get().clone());
                            }
                        },
                        on_click: move |_| {
                            login_info.with_mut(|info| info.username = username.get().clone());
                        }
                    }
                }

                div {
                    MessageInput {
                        itype: "password",
                        message: "{password.get()}",
                        placeholder: "password",
                        error: if password.get().len() > 0 {error.get().as_ref()}else {None},
                        on_input: move |event: FormEvent| {
                            password.set(event.value.clone())
                        },
                        on_keypress: move |event: KeyboardEvent| {
                            if event.code() == keyboard_types::Code::Enter && username.get().len() > 0 && password.get().len() > 0 {
                                login_info.with_mut(|info| info.password = password.get().clone());

                            }
                        },
                        on_click: move |_| {
                            login_info.with_mut(|info| info.password = password.get().clone());
                        }
                    }
                }
            ))
        })
    })
}

pub async fn sync(client: Client, initial_sync_token: Option<String>) -> anyhow::Result<()> {
    let mut sync_settings = SyncSettings::default();

    if let Some(sync_token) = initial_sync_token {
        sync_settings = sync_settings.token(sync_token);
    }

    loop {
        match client.sync_once(sync_settings.clone()).await {
            Ok(response) => {
                persist_sync_token(response.next_batch).await?;
                break;
            }
            Err(error) => {
                info!("An error occurred during initial sync: {error}");
                info!("Trying again…");
            }
        }
    }

    info!("The client is ready! Listening to new messages…");

    Ok(())
}

pub async fn persist_sync_token(sync_token: String) -> anyhow::Result<()> {
    let serialized_session: Result<String, StorageError> =
        <LocalStorage as gloo::storage::Storage>::get("session_file");

    let serialized_session = serialized_session.unwrap();
    let mut full_session: FullSession = serde_json::from_str(&serialized_session)?;

    full_session.sync_token = Some(sync_token);
    let serialized_session = serde_json::to_string(&full_session)?;
    let _ = <LocalStorage as gloo::storage::Storage>::set("session_file", serialized_session);

    Ok(())
}
