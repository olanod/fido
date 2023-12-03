use std::collections::HashMap;

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{translate, i18n::use_i18};
use gloo::storage::LocalStorage;
use log::info;
use crate::{
    components::{
        atoms::{MessageInput, input::InputType, LoadingStatus},
        organisms::{login_form::FormLoginEvent, LoginForm},
    },
    utils::i18n_get_key_value::i18n_get_key_value, services::matrix::matrix::login, hooks::{use_client::use_client, use_init_app::BeforeSession, use_auth::use_auth, use_session::use_session},
};

#[derive(Debug, Clone)]
pub struct LoggedIn(pub bool);

pub enum LoggedInStatus {
    Start,
    Loading,
    Done,
    Persisting,
    LoggedAs(String)
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
    let auth = use_auth(cx);
    let session = use_session(cx);

    let homeserver = use_state(cx, || String::from(""));
    let username = use_state(cx, || String::from(""));
    let password = use_state(cx, || String::from(""));
    let error = use_state(cx, || None);

    let before_session =
        use_shared_state::<BeforeSession>(cx).expect("Unable to use before session");

    let is_loading_loggedin = use_ref::<LoggedInStatus>(cx, || LoggedInStatus::Start);

    let error_invalid_credentials = i18n_get_key_value(
        &i18n_map,
        key_login_chat_errors_invalid_username_password,
    );
    let error_unknown = i18n_get_key_value(
        &i18n_map, key_login_chat_errors_unknown,
    );

    let on_update_homeserver = move || {
        cx.spawn({
            to_owned![homeserver, auth];

            async move {
                auth.set_server(homeserver.get().clone()).await;
            }
        })
    };

    let on_handle_login = move || {
        auth.set_username(username.get().clone(), true);
        auth.set_password(password.get().clone());

        cx.spawn({
            to_owned![auth, session, username, password, is_loading_loggedin, client, error, error_invalid_credentials, error_unknown, homeserver];

            async move {
                is_loading_loggedin.set(LoggedInStatus::Loading);
                let login_config = auth.build();

                match login_config {
                    Ok(info) => {
                        let response = login(
                            &info.server.into(),
                            &info.username,
                            &info.password,
                        )
                        .await;

                        match response {
                            Ok((c, serialized_session)) => {
                                is_loading_loggedin.set(LoggedInStatus::Done);
                                let _x = <LocalStorage as gloo::storage::Storage>::set(
                                    "session_file",
                                    serialized_session,
                                );
        
                                is_loading_loggedin.set(LoggedInStatus::Persisting);
                
                                let _x = session.sync(c.clone(), None).await;
                                
                                let x = c.whoami().await;
                                info!("whoami {:?}", x);
        
                                client.set(crate::MatrixClientState { client: Some(c.clone()) });
                                is_loading_loggedin.set(LoggedInStatus::LoggedAs(c.user_id().unwrap().to_string()));
        
                                auth.set_logged_in(true)
                            }
                            Err(err) => {
                                info!("error from loggin: {:?}", err.to_string());
                                is_loading_loggedin.set(LoggedInStatus::Start);
                                if err
                                    .to_string()
                                    .eq("the server returned an error: [403 / M_FORBIDDEN] Invalid username or password")
                                {
                                    error.set(Some(error_invalid_credentials))
                                } else {
                                    error.set(Some(error_unknown))
                                }

                                homeserver.set(String::new());
                                username.set(String::new());
                                password.set(String::new());
                                
                                auth.reset()
                            }
                        }
                    }
                    Err(e) => {
                        homeserver.set(String::new());
                        username.set(String::new());
                        password.set(String::new());
                        
                        auth.reset()
                    }
                }
            }
        })
    };

    render!(
        if auth.get().data.server.is_none() {
            rsx!(
                LoginForm {
                    title: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_message)}",
                    description: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_description)}",
                    button_text: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_cta)}",
                    emoji: "🛰️",
                    error: if error.get().is_some() { error.get().as_ref() } else { None },
                    on_handle: move |event: FormLoginEvent| match event {
                        FormLoginEvent::FilledForm => on_update_homeserver(),
                        FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
                        FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
                    },
                    body: render!(rsx!(
                        div {
                            MessageInput {
                                message: "{homeserver.get()}",
                                placeholder: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_placeholder)}",
                                error: None,
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
        } else if auth.get().data.username.is_none() || auth.get().data.password.is_none() {
            rsx!(
                LoginForm {
                    title: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_title)}",
                    description: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_description)}",
                    button_text: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_cta)}",
                    emoji: "👋",
                    error: if error.get().is_some() { error.get().as_ref() } else { None },
                    on_handle: move |event: FormLoginEvent| match event {
                        FormLoginEvent::FilledForm => on_handle_login(),
                        FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
                        FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
                    },
                    body: render!(rsx!(
                        div {
                            MessageInput {
                                message: "{username.get()}",
                                placeholder: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_username_placeholder)}",
                                error: None,
                                on_input: move |event: FormEvent| {
                                    username.set(event.value.clone())
                                },
                                on_keypress: move |event: KeyboardEvent| {
                                    if event.code() == keyboard_types::Code::Enter && !username.get().is_empty() {
                                        auth.set_username(username.get().clone(), true)
                                    }
                                },
                                on_click: move |_| {
                                    auth.set_username(username.get().clone(), true)
                                }
                            }
                        }

                        div {
                            MessageInput {
                                itype: InputType::Password,
                                message: "{password.get()}",
                                placeholder: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_password_placeholder)}",
                                error: None,
                                on_input: move |event: FormEvent| {
                                    password.set(event.value.clone())
                                },
                                on_keypress: move |event: KeyboardEvent| {
                                    if event.code() == keyboard_types::Code::Enter && !username.get().is_empty() && !password.get().is_empty() {
                                        // on_handle_login()
                                    }
                                },
                                on_click: move |_| {
                                    auth.set_password(password.get().clone());
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
