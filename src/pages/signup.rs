use std::{collections::HashMap, ops::Deref};

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{i18n::use_i18, translate};
use gloo::storage::{LocalStorage, Storage};
use log::info;
use matrix_sdk::{Error, HttpError};
use ruma::api::{
    client::uiaa::{self, AuthType, UiaaResponse},
    error::{FromHttpResponseError, ServerError},
};
use serde_json::Value;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    components::{
        atoms::{input::InputType, MessageInput, Spinner},
        organisms::{login_form::FormLoginEvent, LoginForm},
    },
    hooks::{
        use_auth::{use_auth, UseAuthState},
        use_client::use_client,
        use_init_app::BeforeSession,
        use_session::use_session,
    },
    pages::login::LoggedInStatus,
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
    let auth = use_auth(cx);
    let session = use_session(cx);

    let homeserver = use_state(cx, || String::from(""));
    let username = use_state(cx, || String::from(""));
    let password = use_state(cx, || String::from(""));
    let error = use_state(cx, || None);

    let before_session =
        use_shared_state::<BeforeSession>(cx).expect("Unable to use before session");

    let flows = use_ref::<Vec<AuthType>>(cx, || vec![]);
    let session_ref = use_ref::<Option<String>>(cx, || None);

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window)]
        fn onloadCallback();
    }

    let on_update_homeserver = move || {
        cx.spawn({
            to_owned![homeserver, auth];

            async move {
                auth.set_server(homeserver.current()).await;
            }
        })
    };

    let on_handle_clear = move || {
        cx.spawn({
            to_owned![homeserver, username, password, auth];

            async move {
                reset_login_info(&auth, &homeserver, &username, &password);
            }
        })
    };

    let is_loading_loggedin = use_ref::<LoggedInStatus>(cx, || LoggedInStatus::Start);

    let on_handle_login = move || {
        auth.set_username(username.get().clone(), false);
        auth.set_password(password.get().clone());

        cx.spawn({
            to_owned![
                auth,
                username,
                password,
                client,
                error,
                flows,
                session_ref,
                homeserver
            ];

            async move {
                let login_config = auth.build();

                let Ok(info) = login_config else {
                    reset_login_info(&auth, &homeserver, &username, &password);
                    return;
                };
                let response =
                    prepare_register(info.server.as_str(), &info.username, &info.password).await;

                if let Err(Error::Http(HttpError::UiaaError(FromHttpResponseError::Server(
                    ServerError::Known(ref f_error),
                )))) = response
                {
                    flow_error(
                        &auth,
                        &homeserver,
                        &username,
                        &password,
                        &session_ref,
                        &error,
                        &f_error,
                        &flows,
                    )
                }

                info!("response {response:?}");
            }
        })
    };

    let on_handle_captcha = move || {
        cx.spawn({
            to_owned![
                auth,
                client,
                error,
                session_ref,
                is_loading_loggedin,
                before_session,
                session,
                homeserver,
                username,
                password
            ];

            async move {
                let recaptcha_token = <LocalStorage as gloo::storage::Storage>::get("recaptcha");
                let session_id = session_ref.read().clone();
                let Ok(token) = recaptcha_token else {
                    info!("token not found");
                    return;
                };

                let Ok(info) = auth.build() else {
                    reset_login_info(&auth, &homeserver, &username, &password);
                    return;
                };

                let response = register(
                    &info.server.to_string(),
                    &info.username,
                    &info.password,
                    Some(token),
                    session_id,
                )
                .await
                .expect("TODO: handle failed registration");

                let Ok((c, serialized_session)) =
                    login(info.server.as_str(), &info.username, &info.password).await
                else {
                    is_loading_loggedin.set(LoggedInStatus::Start);
                    *before_session.write() = BeforeSession::Login;
                    return;
                };

                is_loading_loggedin.set(LoggedInStatus::Done);

                <LocalStorage as gloo::storage::Storage>::set("session_file", serialized_session);

                is_loading_loggedin.set(LoggedInStatus::Persisting);

                session.sync(c.clone(), None).await;

                client.set(crate::MatrixClientState {
                    client: Some(c.clone()),
                });

                is_loading_loggedin.set(LoggedInStatus::LoggedAs(c.user_id().unwrap().to_string()));

                auth.set_logged_in(true);
            }
        })
    };

    render!(
        div {
            class: "page--clamp",
            if auth.get().data.server.is_none() {
                rsx!(LoginForm {
                    title: "{i18n_get_key_value(&i18n_map, key_signup_chat_homeserver_message)}",
                    description: "{i18n_get_key_value(&i18n_map, key_signup_chat_homeserver_description)}",
                    button_text: "{i18n_get_key_value(&i18n_map, key_signup_chat_homeserver_cta)}",
                    emoji: "🛰️",
                    error: error.get().as_ref(),
                    on_handle: move |event: FormLoginEvent| match event {
                        FormLoginEvent::FilledForm => on_update_homeserver(),
                        FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
                        FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
                        FormLoginEvent::ClearData => on_handle_clear(),
                    },
                    body: render!(rsx!(
                        div {
                            MessageInput {
                                message: "{homeserver.get()}",
                                placeholder: "{i18n_get_key_value(&i18n_map, key_signup_chat_homeserver_placeholder)}",
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
                })
            } else if auth.get().data.username.is_none() || auth.get().data.password.is_none() {
                rsx!(LoginForm {
                    title: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_title)}",
                    description: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_description)}",
                    button_text: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_cta)}",
                    emoji: "✍️",
                    error: error.get().as_ref(),
                    on_handle: move |event: FormLoginEvent| match event {
                        FormLoginEvent::FilledForm => on_handle_login(),
                        FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
                        FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
                        FormLoginEvent::ClearData => on_handle_clear(),
                    },
                    body: render!(rsx!(
                        div {
                            MessageInput {
                                message: "{username.get()}",
                                placeholder: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_username_placeholder)}",
                                error: None,
                                on_input: move |event: FormEvent| {
                                    username.set(event.value.clone())
                                },
                                on_keypress: move |event: KeyboardEvent| {
                                    if event.code() == keyboard_types::Code::Enter && !username.get().is_empty() {
                                        auth.set_username(username.get().clone(), false)
                                    }
                                },
                                on_click: move |_| {
                                    auth.set_username(username.get().clone(), false)
                                }
                            }
                        }

                        div {
                            MessageInput {
                                itype: InputType::Password,
                                message: "{password.get()}",
                                placeholder: "{i18n_get_key_value(&i18n_map, key_signup_chat_credentials_password_placeholder)}",
                                error: None,
                                on_input: move |event: FormEvent| {
                                    password.set(event.value.clone())
                                },
                                on_keypress: move |event: KeyboardEvent| {
                                    if event.code() == keyboard_types::Code::Enter && !username.get().is_empty() && !password.get().is_empty() {
                                        auth.set_password(password.get().clone());
                                    }
                                },
                                on_click: move |_| {
                                    auth.set_password(password.get().clone());
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
                                    error: error.get().as_ref(),
                                    on_handle: move |event: FormLoginEvent| match event {
                                        FormLoginEvent::FilledForm => on_handle_captcha(),
                                        FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
                                        FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
                                        FormLoginEvent::ClearData => on_handle_clear()

                                    },
                                    body: render!(rsx!(div {
                                        class: "signup__flow",
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
            }
        }
    )
}

fn flow_error(
    auth: &UseAuthState,
    homeserver: &UseState<String>,
    username: &UseState<String>,
    password: &UseState<String>,
    session_ref: &UseRef<Option<String>>,
    error: &UseState<Option<String>>,
    f_error: &UiaaResponse,
    flows: &UseRef<Vec<AuthType>>,
) {
    match f_error {
        uiaa::UiaaResponse::AuthResponse(uiaa_info) => {
            let completed = &uiaa_info.completed;
            let mut flows_to_complete: Vec<AuthType> = vec![];

            uiaa_info.flows[0].stages.iter().for_each(|f| {
                if completed.iter().find(|e| *e == f).is_none() {
                    flows_to_complete.push(f.clone());
                }
                session_ref.set(uiaa_info.session.clone());

                match f {
                    AuthType::ReCaptcha => {
                        let params = uiaa_info.params.deref().get();
                        let uiaa_response = serde_json::from_str(params);

                        set_site_key(uiaa_response)
                    }
                    _ => {
                        info!("Unsuported flow");
                    }
                }
            });

            flows.set(flows_to_complete)
        }
        uiaa::UiaaResponse::MatrixError(e) => {
            reset_login_info(&auth, &homeserver, &username, &password);

            error.set(Some(e.message.clone()));
        }
        _ => {
            reset_login_info(&auth, &homeserver, &username, &password);

            error.set(Some(String::from("Unspecified error")));
        }
    }
}

fn reset_login_info(
    auth: &UseAuthState,
    homeserver: &UseState<String>,
    username: &UseState<String>,
    password: &UseState<String>,
) {
    homeserver.set(String::from(""));
    username.set(String::from(""));
    password.set(String::from(""));
    auth.reset();
}

fn set_site_key(uiaa_response: Result<HashMap<String, Value>, serde_json::Error>) {
    match uiaa_response {
        Ok(u) => {
            let m = u.get("m.login.recaptcha");

            if let Some(Value::Object(ref recaptcha)) = m {
                if let Some(Value::String(public_key)) = recaptcha.get("public_key") {
                    gloo::storage::LocalStorage::set("sitekey", public_key.clone());
                }
            }
        }
        Err(_) => todo!(),
    };
}
