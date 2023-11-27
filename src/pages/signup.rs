use std::{collections::HashMap, ops::Deref};

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{i18n::use_i18, translate};
use gloo::storage::{LocalStorage, Storage};
use log::info;
use matrix_sdk::{Client, Error, HttpError};
use ruma::api::{
    client::uiaa::{self, AuthType},
    error::{FromHttpResponseError, ServerError},
};
use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    components::{
        atoms::{input::InputType, MessageInput, Spinner},
        organisms::{login_form::FormLoginEvent, LoginForm},
    },
    hooks::{use_client::use_client, use_init_app::BeforeSession},
    pages::login::{sync, LoggedIn, LoggedInStatus, LoginInfo},
    services::matrix::matrix::{login, prepare_register, register},
    utils::i18n_get_key_value::i18n_get_key_value,
};

pub fn Signup(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    // homeserver
    let key_signup_chat_homeserver_message = "signup-chat-homeserver-message";
    let key_signup_chat_homeserver_description = "signup-chat-homeserver-description";
    let key_signup_chat_homeserver_placeholder = "signup-chat-homeserver-placeholder";
    let key_signup_chat_homeserver_cta = "signup-chat-homeserver-cta";

    // credentials
    let key_signup_chat_credentials_title = "signup-chat-credentials-title";
    let key_signup_chat_credentials_description = "signup-chat-credentials-description";
    let key_signup_chat_credentials_username_message = "signup-chat-credentials-username-message";
    let key_signup_chat_credentials_username_placeholder =
        "signup-chat-credentials-username-placeholder";
    let key_signup_chat_credentials_password_message = "signup-chat-credentials-password-message";
    let key_signup_chat_credentials_password_placeholder =
        "signup-chat-credentials-password-placeholder";
    let key_signup_chat_credentials_cta = "signup-chat-credentials-cta";

    // captcha
    let key_signup_chat_captcha_title = "signup-chat-captcha-title";
    let key_signup_chat_captcha_description = "signup-chat-captcha-description";
    let key_signup_chat_captcha_cta = "signup-chat-captcha-cta";

    let i18n_map = HashMap::from([
        // homeserver
        (
            key_signup_chat_homeserver_message,
            translate!(i18, "signup.chat_steps.homeserver.message"),
        ),
        (
            key_signup_chat_homeserver_description,
            translate!(i18, "signup.chat_steps.homeserver.description"),
        ),
        (
            key_signup_chat_homeserver_placeholder,
            translate!(i18, "signup.chat_steps.homeserver.placeholder"),
        ),
        (
            key_signup_chat_homeserver_cta,
            translate!(i18, "signup.chat_steps.homeserver.cta"),
        ),
        (
            key_signup_chat_credentials_title,
            translate!(i18, "signup.chat_steps.credentials.title"),
        ),
        // credentials
        (
            key_signup_chat_credentials_description,
            translate!(i18, "signup.chat_steps.credentials.description"),
        ),
        (
            key_signup_chat_credentials_username_message,
            translate!(i18, "signup.chat_steps.credentials.username.message"),
        ),
        (
            key_signup_chat_credentials_username_placeholder,
            translate!(i18, "signup.chat_steps.credentials.username.placeholder"),
        ),
        (
            key_signup_chat_credentials_password_message,
            translate!(i18, "signup.chat_steps.credentials.password.message"),
        ),
        (
            key_signup_chat_credentials_password_placeholder,
            translate!(i18, "signup.chat_steps.credentials.password.placeholder"),
        ),
        (
            key_signup_chat_credentials_cta,
            translate!(i18, "signup.chat_steps.credentials.cta"),
        ),
        // captcha
        (
            key_signup_chat_captcha_title,
            translate!(i18, "signup.chat_steps.captcha.title"),
        ),
        (
            key_signup_chat_captcha_description,
            translate!(i18, "signup.chat_steps.captcha.description"),
        ),
        (
            key_signup_chat_captcha_cta,
            translate!(i18, "signup.chat_steps.captcha.cta"),
        ),
    ]);

    let client = use_client(cx);
    let homeserver = use_state(cx, || String::from(""));
    let username = use_state(cx, || String::from(""));
    let password = use_state(cx, || String::from(""));
    let error = use_state(cx, || None);

    let logged_in = use_shared_state::<LoggedIn>(cx).expect("Unable to use Logged in");
    let before_session =
        use_shared_state::<BeforeSession>(cx).expect("Unable to use before session");

    let flows = use_ref::<Vec<AuthType>>(cx, || vec![]);
    let session = use_ref::<Option<String>>(cx, || None);

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window)]
        fn onloadCallback();
    }

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

    let is_loading_loggedin = use_ref::<LoggedInStatus>(cx, || LoggedInStatus::Start);

    let on_handle_login = move || {
        login_info.with_mut(|info| info.username = username.get().clone());
        login_info.with_mut(|info| info.password = password.get().clone());

        cx.spawn({
            to_owned![logged_in, login_info, username, password, client, error, flows, session];

            info!("{:?}", login_info.read());

            async move {
                let response = prepare_register(
                    login_info.read().homeserver.clone(),
                    login_info.read().username.clone(),
                    login_info.read().password.clone(),
                )
                .await;

                match response {
                    Ok(_) => todo!(),
                    Err(ref error) => match error {
                        Error::Http(HttpError::UiaaError(FromHttpResponseError::Server(
                            ServerError::Known(x),
                        ))) => match x {
                            uiaa::UiaaResponse::AuthResponse(y) => {
                                let completed = &y.completed;
                                let mut flows_to_complete: Vec<AuthType> = vec![];

                                y.flows[0].stages.iter().for_each(|f| {
                                    if completed.iter().find(|e| *e == f).is_none() {
                                        flows_to_complete.push(f.clone());
                                    }
                                    session.set(y.session.clone());

                                    match f {
                                        AuthType::ReCaptcha => {
                                            let x = y.params.deref().get();
                                            let uiaa_response: Result<
                                                HashMap<String, _>,
                                                serde_json::Error,
                                            > = serde_json::from_str(x);

                                            match uiaa_response {
                                                Ok(u) => {
                                                    let m = u.get("m.login.recaptcha");

                                                    if let Some(Value::Object(ref recaptcha)) = m {
                                                        if let Some(Value::String(public_key)) =
                                                            recaptcha.get("public_key")
                                                        {
                                                            gloo::storage::LocalStorage::set(
                                                                "sitekey",
                                                                public_key.clone(),
                                                            );
                                                        }
                                                    }
                                                }
                                                Err(_) => todo!(),
                                            };
                                        }
                                        _ => {
                                            info!("Unsuported flow");
                                        }
                                    }
                                });

                                flows.set(flows_to_complete)
                            }
                            _ => todo!(),
                        },
                        _ => {}
                    },
                }

                info!("response {response:?}");
            }
        })
    };

    let on_handle_captcha = move || {
        cx.spawn({
            to_owned![
                login_info,
                client,
                error,
                session,
                is_loading_loggedin,
                logged_in,
                before_session
            ];

            async move {
                let recaptcha_token = <LocalStorage as gloo::storage::Storage>::get("recaptcha");
                let session_id = session.read().clone();
                match recaptcha_token {
                    Ok(token) => {
                        let response = register(
                            login_info.read().homeserver.clone(),
                            login_info.read().username.clone(),
                            login_info.read().password.clone(),
                            Some(token),
                            session_id,
                        )
                        .await;

                        match response {
                            Ok((ref c, ref serialized_session)) => {
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

                                        client.set(crate::MatrixClientState {
                                            client: Some(c.clone()),
                                        });
                                        is_loading_loggedin.set(LoggedInStatus::LoggedAs(
                                            c.user_id().unwrap().to_string(),
                                        ));

                                        logged_in.write().is_logged_in = true;
                                    }
                                    Err(err) => {
                                        info!("{:?}", err.to_string());
                                        is_loading_loggedin.set(LoggedInStatus::Start);
                                        *before_session.write() = BeforeSession::Login
                                    }
                                }
                            }
                            Err(error) => todo!(),
                        }
                    }
                    Err(_) => {
                        info!("token not found");
                    }
                }
            }
        })
    };

    render!(if login_info.read().homeserver.len() == 0 {
        rsx!(LoginForm {
            title: "{i18n_get_key_value(&i18n_map, key_signup_chat_homeserver_message)}",
            description: "{i18n_get_key_value(&i18n_map, key_signup_chat_homeserver_description)}",
            button_text: "{i18n_get_key_value(&i18n_map, key_signup_chat_homeserver_cta)}",
            emoji: "🛰️",
            on_handle: move |event: FormLoginEvent| match event {
                FormLoginEvent::FilledForm => on_update_homeserver(),
                FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
                FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
            },
            body: render!(rsx!(
                div {
                    MessageInput {
                        message: "{homeserver.get()}",
                        placeholder: "{i18n_get_key_value(&i18n_map, key_signup_chat_homeserver_placeholder)}",
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
            title: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_title)}",
            description: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_description)}",
            button_text: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_cta)}",
            emoji: "✍️",
            on_handle: move |event: FormLoginEvent| match event {
                FormLoginEvent::FilledForm => on_handle_login(),
                FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
                FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
            },
            body: render!(rsx!(
                div {
                    MessageInput {
                        message: "{username.get()}",
                        placeholder: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_username_placeholder)}",
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
                        placeholder: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_password_placeholder)}",
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
    } else if flows.read().len() > 0 {
        let f = flows.read();
        let flows = f.clone();

        let mut element = rsx!(div {});

        for flow in flows.iter() {
            let i18n_map = i18n_map.clone();
            element = match flow {
                AuthType::ReCaptcha => rsx!(
                    div {
                        onmounted: move |_| onloadCallback(),
                        LoginForm {
                            title: "{i18n_get_key_value(&i18n_map, key_signup_chat_captcha_title)}",
                            description: "{i18n_get_key_value(&i18n_map, key_signup_chat_captcha_description)}",
                            button_text: "{i18n_get_key_value(&i18n_map, key_signup_chat_captcha_cta)}",
                            emoji: "✍️",
                            on_handle: move |event: FormLoginEvent| match event {
                                FormLoginEvent::FilledForm => on_handle_captcha(),
                                FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
                                FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
                            },
                            body: render!(rsx!(div {
                                style: "
                                    display: flex;
                                    justify-content: center;
                                ",
                                id: "recaptcha-container",
                            }))
                        }
                    }
                ),
                _ => rsx!(div {}),
            };
        }

        element
    } else {
        rsx!(
            div {
                class: "column spinner-dual-ring--center",
                Spinner {}
            }
        )
    })
}
