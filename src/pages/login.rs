use std::collections::HashMap;

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{translate, i18n::use_i18};
use gloo::storage::{LocalStorage, errors::StorageError};
use log::info;
use matrix_sdk::{Client, config::SyncSettings};

use crate::{
    components::{
        atoms::{MessageInput, input::InputType, Spinner},
        organisms::{login_form::FormLoginEvent, LoginForm},
    },
    utils::i18n_get_key_value::i18n_get_key_value, services::matrix::matrix::{login, FullSession}, hooks::use_client::use_client,
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

pub enum LoggedInStatus {
    Start,
    Loading,
    Done,
    Persisting,
    LoggedAs(String)
}

#[inline_props]
fn LoadingStatus(cx: Scope, text: String) -> Element {
    cx.render({
        rsx!(
            div {
                class: "column spinner-dual-ring--center",
                Spinner {}

                p {
                    style: "color: var(--text-1)",
                    "{text}"
                }
            }
        )
    })
}

pub fn Login(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    // Claves para el fragmento "chat_steps" dentro de "login"
    let key_login_chat_homeserver_message = "login-chat-homeserver-message";
    let key_login_chat_homeserver_description = "login-chat-homeserver-description";
    let key_login_chat_homeserver_placeholder = "login-chat-homeserver-placeholder";
    let key_login_chat_homeserver_cta = "login-chat-homeserver-cta";

    let key_login_chat_credentials_description = "login-chat-credentials-description";
    let key_login_chat_credentials_title = "login-chat-credentials-title";

    // Claves para el fragmento "username" dentro de "chat_steps.credentials"
    let key_login_chat_credentials_username_message = "login-chat-credentials-username-message";
    let key_login_chat_credentials_username_placeholder = "login-chat-credentials-username-placeholder";

    // Claves para el fragmento "password" dentro de "chat_steps.credentials"
    let key_login_chat_credentials_password_message = "login-chat-credentials-password-message";
    let key_login_chat_credentials_password_placeholder = "login-chat-credentials-password-placeholder";

    let key_login_chat_credentials_cta = "login-chat-credentials-cta";

    // Claves para el fragmento "messages" dentro de "chat_steps"
    let key_login_chat_messages_validating = "login-chat-messages-validating";
    let key_login_chat_messages_welcome = "login-chat-messages-welcome";

    // Claves para el fragmento "chat_errors" dentro de "login"
    let key_login_chat_errors_invalid_url = "login-chat-errors-invalid-url";
    let key_login_chat_errors_unknown = "login-chat-errors-unknown";
    let key_login_chat_errors_invalid_username_password = "login-chat-errors-invalid-username-password";

    let i18n_map = HashMap::from([
        // Traducciones para el fragmento "chat_steps.homeserver" dentro de "login"
        (key_login_chat_homeserver_message, translate!(i18, "login.chat_steps.homeserver.message")),
        (key_login_chat_homeserver_description, translate!(i18, "login.chat_steps.homeserver.description")),
        (key_login_chat_homeserver_placeholder, translate!(i18, "login.chat_steps.homeserver.placeholder")),
        (key_login_chat_homeserver_cta, translate!(i18, "login.chat_steps.homeserver.cta")),

        (key_login_chat_credentials_title, translate!(i18, "login.chat_steps.credentials.title")),
        // Traducciones para el fragmento "chat_steps.credentials" dentro de "login"
        (key_login_chat_credentials_description, translate!(i18, "login.chat_steps.credentials.description")),

        // Traducciones para el fragmento "username" dentro de "chat_steps.credentials"
        (key_login_chat_credentials_username_message, translate!(i18, "login.chat_steps.credentials.username.message")),
        (key_login_chat_credentials_username_placeholder, translate!(i18, "login.chat_steps.credentials.username.placeholder")),

        // Traducciones para el fragmento "password" dentro de "chat_steps.credentials"
        (key_login_chat_credentials_password_message, translate!(i18, "login.chat_steps.credentials.password.message")),
        (key_login_chat_credentials_password_placeholder, translate!(i18, "login.chat_steps.credentials.password.placeholder")),
        (key_login_chat_credentials_cta, translate!(i18, "login.chat_steps.credentials.cta")),

        // Traducciones para el fragmento "messages" dentro de "chat_steps"
        (key_login_chat_messages_validating, translate!(i18, "login.chat_steps.messages.validating")),
        (key_login_chat_messages_welcome, translate!(i18, "login.chat_steps.messages.welcome")),

        // Traducciones para el fragmento "chat_errors" dentro de "login"
        (key_login_chat_errors_invalid_url, translate!(i18, "login.chat_errors.invalid_url")),
        (key_login_chat_errors_unknown, translate!(i18, "login.chat_errors.unknown")),
        (key_login_chat_errors_invalid_username_password, translate!(i18, "login.chat_errors.invalid_username_password")),
    ]);

    let client = use_client(cx);
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

    let is_loading_loggedin = use_ref::<LoggedInStatus>(cx, || LoggedInStatus::Start);

    let on_update_homeserver = move || {
        cx.spawn({
            to_owned![login_info, homeserver, error, is_loading_loggedin];

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

    let error_invalid_credentials = i18n_get_key_value(
        &i18n_map,
        key_login_chat_errors_invalid_username_password,
    );
    let error_unknown = i18n_get_key_value(
        &i18n_map, key_login_chat_errors_unknown,
    );

    let on_handle_login = move || {
        login_info.with_mut(|info| info.username = username.get().clone());
        login_info.with_mut(|info| info.password = password.get().clone());

        cx.spawn({
            to_owned![logged_in, login_info, username, password, is_loading_loggedin, client, error, error_invalid_credentials, error_unknown];

            info!("{:?}",login_info.read());

            async move {
                is_loading_loggedin.set(LoggedInStatus::Loading);
                let response = login(
                    login_info.read().homeserver.clone(),
                    login_info.read().username.clone(),
                    login_info.read().password.clone(),
                )
                .await;
        
                match response {
                    Ok((c, serialized_session)) => {
                        is_loading_loggedin.set(LoggedInStatus::Done);
                        let x = <LocalStorage as gloo::storage::Storage>::set(
                            "session_file",
                            serialized_session,
                        );

                        is_loading_loggedin.set(LoggedInStatus::Persisting);
        
                        info!("Session persisted in {:?}", x);
        
                        let x = sync(c.clone(), None).await;
        
                        info!("new session {:?}", x);
        
                        
                        let x = c.whoami().await;
                        info!("whoami {:?}", x);

                        client.set(crate::MatrixClientState { client: Some(c.clone()) });
                        is_loading_loggedin.set(LoggedInStatus::LoggedAs(c.user_id().unwrap().to_string()));

                        logged_in.write().is_logged_in = true;
                    }
                    Err(err) => {
                        info!("{:?}", err.to_string());
                        is_loading_loggedin.set(LoggedInStatus::Start);
                        if err
                            .to_string()
                            .eq("the server returned an error: [403 / M_FORBIDDEN] Invalid username or password")
                        {
                            error.set(Some(error_invalid_credentials))
                        } else {
                            error.set(Some(error_unknown))
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

    render!(
        if login_info.read().homeserver.len() == 0 {
            rsx!(
                LoginForm {
                    title: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_message)}",
                    description: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_description)}",
                    button_text: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_cta)}",
                    emoji: "🛰️",
                    on_handle: move |_| { on_update_homeserver() },
                    body: render!(rsx!(
                        div {
                            MessageInput {
                                message: "{homeserver.get()}",
                                placeholder: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_placeholder)}",
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
                }
            )
        } else if login_info.read().username.len() == 0 || login_info.read().password.len() == 0 {
            rsx!(
                LoginForm {
                    title: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_title)}",
                    description: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_description)}",
                    button_text: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_cta)}",
                    emoji: "👋",
                    on_handle: move |_: FormLoginEvent| {
                        on_handle_login()
                    },
                    body: render!(rsx!(
                        div {
                            MessageInput {
                                message: "{username.get()}",
                                placeholder: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_username_placeholder)}",
                                error: if username.get().len() > 0 {error.get().as_ref()}else {None},
                                on_input: move |event: FormEvent| {
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
                                itype: InputType::Password,
                                message: "{password.get()}",
                                placeholder: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_password_placeholder)}",
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
                }
            )
        } else {
            match &*is_loading_loggedin.read() {
                LoggedInStatus::Loading => {
                    rsx!(
                        LoadingStatus {text: "Estamos verificando tus datos".to_string()}
                    )
                }
                LoggedInStatus::LoggedAs(user) => {
                    rsx!(
                        LoadingStatus {text: "Te damos la bienvenida {user}".to_string()}
                    )
                },
                LoggedInStatus::Done => {
                    rsx!(
                        LoadingStatus {text: "Te damos la bienvenida ".to_string()}
                    )
                }
                LoggedInStatus::Persisting => {
                    rsx!(
                        LoadingStatus {text: "Guardando tu sesion".to_string()}
                    )
                }
                _ => {
                    rsx!(div{})
                }
                
            }
        }
    )
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
                info!("Trying again from login…");
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
