#![allow(non_snake_case)]
use chat::components::atoms::{LoadingStatus, Notification, Spinner};
use chat::hooks::use_auth::use_auth;
use chat::hooks::use_client::use_client;
use chat::hooks::use_init_app::{use_init_app, BeforeSession};
use chat::hooks::use_notification::{use_notification, NotificationType};
use chat::hooks::use_session::use_session;
use chat::pages::login::{LoggedIn, Login};
use chat::pages::route::Route;
use chat::pages::signup::Signup;
use chat::utils::get_element;
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
use ruma::api::client::filter::{Filter, FilterDefinition, RoomEventFilter, RoomFilter};
use ruma::api::client::sync::sync_events;
use ruma::events::EventType;
use std::str::FromStr;
use std::time::Duration;
use web_sys::console::info;
use web_sys::window;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

static EN_US: &str = include_str!("./locales/en-US.json");
static ES_ES: &str = include_str!("./locales/es-ES.json");

fn App(cx: Scope) -> Element {
    use_init_i18n(
        cx,
        "es-ES".parse().expect("can't parse es-ES language"),
        "es-ES".parse().expect("can't parse es-ES language"),
        || {
            let en_us = Language::from_str(EN_US).expect("can't get EN_US language");
            let es_es = Language::from_str(ES_ES).expect("can't get ES_ES language");
            vec![en_us, es_es]
        },
    );

    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(static_login_form) = document.get_element_by_id("static-login-form") {
                if let Some(parent) = static_login_form.parent_node() {
                    parent
                        .remove_child(&static_login_form)
                        .expect("Failed to remove element");
                }
            }
        }
    }

    use_init_app(cx);

    let client = use_client(cx);
    let auth = use_auth(cx);
    let session = use_session(cx);
    let notification = use_notification(cx);
    let i18 = use_i18(cx);

    let matrix_client =
        use_shared_state::<MatrixClientState>(cx).expect("Unable to use matrix client");
    let before_session =
        use_shared_state::<BeforeSession>(cx).expect("Unable to use before session");

    let restoring_session = use_ref::<bool>(cx, || true);

    use_coroutine(cx, |_: UnboundedReceiver<MatrixClientState>| {
        to_owned![client, auth, restoring_session, session];

        async move {
            let c = create_client("https://matrix.org").await;

            client.set(MatrixClientState {
                client: Some(c.clone()),
            });

            let serialized_session: Result<String, StorageError> =
                <LocalStorage as gloo::storage::Storage>::get("session_file");

            if let Ok(s) = serialized_session {
                let (c, sync_token) = restore_session(&s)
                    .await
                    .expect("can't restore session: main");

                client.set(MatrixClientState {
                    client: Some(c.clone()),
                });

                let x = session.sync(c.clone(), sync_token).await;

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
                            let key_main_loading_title = translate!(
                                i18,
                                "main.loading.title"
                            );
                            rsx!(
                                LoadingStatus {
                                    text: "{key_main_loading_title}",
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
