#![allow(non_snake_case)]
use chat::components::atoms::{LoadingStatus, Notification, Spinner};
use chat::hooks::use_auth::use_auth;
use chat::hooks::use_client::use_client;
use chat::hooks::use_init_app::{use_init_app, BeforeSession};
use chat::hooks::use_notification::{use_notification, NotificationType};
use chat::pages::login::{LoggedIn, Login};
use chat::pages::route::Route;
use chat::pages::signup::Signup;
use chat::MatrixClientState;
use dioxus::prelude::*;
use dioxus_router::prelude::Router;
use gloo::storage::errors::StorageError;
use gloo::storage::LocalStorage;
use log::{info, LevelFilter};

use chat::services::matrix::matrix::*;
use dioxus_std::{i18n::*, translate};
use matrix_sdk::config::SyncSettings;
use matrix_sdk::ruma::exports::serde_json;
use matrix_sdk::Client;
use std::str::FromStr;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

static EN_US: &str = include_str!("./locales/en-US.json");
static ES_ES: &str = include_str!("./locales/es-ES.json");

fn App(cx: Scope) -> Element {
    use_init_i18n(
        cx,
        "es-ES".parse().unwrap(),
        "es-ES".parse().unwrap(),
        || {
            let en_us = Language::from_str(EN_US).unwrap();
            let es_es = Language::from_str(ES_ES).unwrap();
            vec![en_us, es_es]
        },
    );

    use_init_app(cx);

    let client = use_client(cx);
    let auth = use_auth(cx);
    let notification = use_notification(cx);
    let i18 = use_i18(cx);

    let matrix_client = use_shared_state::<MatrixClientState>(cx).unwrap();
    let before_session =
        use_shared_state::<BeforeSession>(cx).expect("Unable to use before session");

    let restoring_session = use_ref::<bool>(cx, || true);

    use_coroutine(cx, |_: UnboundedReceiver<MatrixClientState>| {
        to_owned![client, auth, restoring_session];

        async move {
            let c = create_client(String::from("https://matrix.org")).await;

            client.set(MatrixClientState {
                client: Some(c.clone()),
            });

            let serialized_session: Result<String, StorageError> =
                <LocalStorage as gloo::storage::Storage>::get("session_file");

            if let Ok(s) = serialized_session {
                let (c, sync_token) = restore_session(&s).await.unwrap();

                client.set(MatrixClientState {
                    client: Some(c.clone()),
                });
                let x = sync(c.clone(), sync_token).await;

                auth.set_logged_in(true);

                info!("old session {:?}", x);
                restoring_session.set(false);
            } else {
                restoring_session.set(false);
                info!("else restoring ");
            }
        }
    });

    render! {
        if notification.get().show {
            rsx!(
                Notification {
                    title: "{notification.get().title}",
                    body: "{notification.get().body}",
                    on_click: move |_| {
                       match notification.get().handle.value {
                            NotificationType::Click => {

                            },
                            NotificationType::AcceptSas(sas, redirect) => {
                                cx.spawn({
                                    async move {
                                        let x = sas.accept().await;
                                        todo!()
                                    }
                                });
                            }
                            NotificationType::None => {

                            }
                       }
                    }
                }
            )
        }
        rsx!(
            match &matrix_client.read().client {
                Some(_) => {
                    rsx!(div {
                        class: "page",
                        if auth.is_logged_in().0 {
                            rsx!(
                                section {
                                    class: "chat",
                                    Router::<Route> {}
                                }
                            )
                        } else if *restoring_session.read() {
                            rsx!(
                                LoadingStatus {
                                    text: translate!(
                                        i18,
                                        "main.loading.title"
                                    ),
                                }
                            )
                        } else {
                            match *before_session.read() {
                                BeforeSession::Login => rsx!(
                                    section {
                                        class: "login",
                                        Login {}
                                    }
                                ),
                                BeforeSession::Signup => rsx!(
                                    section {
                                        class: "login",
                                        Signup {}
                                    }
                                )
                            }
                        }
                    })
                }
                None => rsx!(
                    div {
                        class: "spinner-dual-ring--center",
                        Spinner {}
                    }
                ),
            }
        )
    }
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
