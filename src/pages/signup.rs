use std::{collections::HashMap, ops::Deref};

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{i18n::use_i18, translate};
use futures_util::TryFutureExt;
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
};

pub fn Signup() -> Element {
    let i18 = use_i18();

    let mut client = use_client();
    let mut auth = use_auth();
    let mut session = use_session();

    let mut homeserver = use_signal(|| String::from(""));
    let mut username = use_signal(|| String::from(""));
    let mut password = use_signal(|| String::from(""));
    let mut error = use_signal(|| None);

    let mut before_session = consume_context::<Signal<BeforeSession>>();

    let mut flows = use_signal::<Vec<AuthType>>(|| vec![]);
    let mut session_ref = use_signal::<Option<String>>(|| None);

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = window)]
        fn onloadCallback();
    }

    let mut on_update_homeserver = move || {
        spawn({
            async move {
                if let Err(e) = auth.set_server(&homeserver()).await {
                    log::warn!("Failed to set server: {e:?}");
                }
            }
        });
    };

    let on_handle_clear = move || {
        spawn({
            async move {
                reset_login_info(&mut auth, &mut homeserver, &mut username, &mut password);
            }
        });
    };

    let mut is_loading_loggedin = use_signal::<LoggedInStatus>(|| LoggedInStatus::Start);

    let mut on_handle_error = move |e: SignupError| {
        let message_error = match e {
            SignupError::Unknown => translate!(i18, "signup.errors.unknown"),
            SignupError::Server => translate!(i18, "signup.errors.server"),
            SignupError::FlowNotFound => translate!(i18, "signup.errors.flow_not_found"),
            SignupError::RegisterFailed => translate!(i18, "signup.errors.register_failed"),
            SignupError::LoginFailed => translate!(i18, "signup.errors.login_failed"),
            SignupError::SyncFailed => translate!(i18, "chat.common.error.sync"),
            SignupError::SessionFile => translate!(i18, "chat.common.error.persist"),
            SignupError::UnsupportedFlow => translate!(i18, "signup.errors.unsupported_flow"),
            SignupError::KeyRecaptcha | SignupError::SetSiteKey => {
                translate!(i18, "signup.errors.key_recaptcha")
            }
        };
        reset_login_info(&mut auth, &mut homeserver, &mut username, &mut password);

        error.set(Some(message_error));
    };

    let mut on_handle_login = move || {
        auth.set_username(&username(), false);
        auth.set_password(&password());

        spawn({
            async move {
                let info = auth.build().map_err(|_| SignupError::Server)?;

                let response =
                    prepare_register(info.server.as_str(), &info.username, &info.password).await;

                if let Err(Error::Http(HttpError::UiaaError(FromHttpResponseError::Server(
                    ServerError::Known(ref f_error),
                )))) = response
                {
                    flow_error(
                        &mut auth,
                        &mut homeserver,
                        &mut username,
                        &mut password,
                        &mut session_ref,
                        &f_error,
                        &mut flows,
                    )?;
                } else {
                    return Err(SignupError::Unknown);
                }

                info!("response {response:?}");
                Ok::<(), SignupError>(())
            }
            .unwrap_or_else(on_handle_error)
        });
    };

    let mut on_handle_captcha = move || {
        spawn({
            async move {
                let token = <LocalStorage as gloo::storage::Storage>::get::<String>("recaptcha")
                    .map_err(|_| SignupError::KeyRecaptcha)?;
                let session_id = session_ref.read().clone();

                let info = auth.build().map_err(|_| SignupError::RegisterFailed)?;

                register(
                    &info.server.to_string(),
                    &info.username,
                    &info.password,
                    Some(token),
                    session_id,
                )
                .await
                .map_err(|e| {
                    log::info!("{:?}", e);
                    SignupError::RegisterFailed
                })?;

                let (c, serialized_session) =
                    login(info.server.as_str(), &info.username, &info.password)
                        .await
                        .map_err(|_| {
                            is_loading_loggedin.set(LoggedInStatus::Start);
                            *before_session.write() = BeforeSession::Login;

                            SignupError::LoginFailed
                        })?;

                is_loading_loggedin.set(LoggedInStatus::Done);

                <LocalStorage as gloo::storage::Storage>::set("session_file", serialized_session)
                    .map_err(|_| SignupError::SessionFile)?;

                is_loading_loggedin.set(LoggedInStatus::Persisting);

                session
                    .sync(c.clone(), None)
                    .await
                    .map_err(|_| SignupError::SyncFailed)?;

                client.set(crate::MatrixClientState {
                    client: Some(c.clone()),
                });

                auth.set_logged_in(true);

                Ok::<(), SignupError>(())
            }
            .unwrap_or_else(on_handle_error)
        });
    };

    let mut on_action_form = move |event: FormLoginEvent, func: &mut dyn FnMut()| match event {
        FormLoginEvent::FilledForm => func(),
        FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
        FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
        FormLoginEvent::Guest => *before_session.write() = BeforeSession::Guest,
        FormLoginEvent::ClearData => on_handle_clear(),
    };

    rsx!(
        div { class: "page--clamp",
            if auth.get().data.server.is_none() {
                LoginForm {
                    title: translate!(i18, "signup.chat_steps.homeserver.message"),
                    description: translate!(i18, "signup.chat_steps.homeserver.description"),
                    button_text: translate!(i18, "signup.chat_steps.homeserver.cta"),
                    emoji: "🛰️",
                    status: None,
                    error: error(),
                    on_handle: move |event: FormLoginEvent| {
                        on_action_form(event, &mut on_update_homeserver)
                    },
                    body: rsx!(
                        div {
                            MessageInput {
                                message: "{homeserver()}",
                                placeholder: translate!(i18, "signup.chat_steps.homeserver.placeholder"),
                                error: None,
                                on_input: move |event: FormEvent| {
                                    homeserver.set(event.value().clone())
                                },
                                on_keypress: move |event: KeyboardEvent| {
                                    if event.code() == keyboard_types::Code::Enter && ! homeserver().is_empty() {
                                        on_update_homeserver()
                                    }
                                },
                                on_click: move |_| {
                                    on_update_homeserver()
                                }
                            }
                        }
                    )
                }
            } else if auth.get().data.username.is_none() || auth.get().data.password.is_none() {
                LoginForm {
                    title: translate!(i18, "signup.chat_steps.credentials.title"),
                    description: translate!(i18, "signup.chat_steps.credentials.description"),
                    button_text: translate!(i18, "signup.chat_steps.homeserver.cta"),
                    emoji: "✍️",
                    status: None,
                    error: error(),
                    on_handle: move |event: FormLoginEvent| {
                        on_action_form(event, &mut on_handle_login)
                    },
                    body: rsx!(
                        div {
                            MessageInput {
                                message: "{username()}",
                                placeholder: translate!(i18, "signup.chat_steps.credentials.username.placeholder"),
                                error: None,
                                on_input: move |event: FormEvent| {
                                    username.set(event.value().clone());
                                },
                                on_keypress: move |event : KeyboardEvent| {
                                    if event.code() == keyboard_types::Code::Enter && ! username().is_empty() {
                                        auth.set_username(&username(), false);
                                    }
                                },
                                on_click: move |_| {
                                    auth.set_username(& username(),false);
                                }
                            }
                        }
                        div {
                            MessageInput {
                                itype: InputType::Password,
                                message: "{password()}",
                                placeholder: translate!(i18, "signup.chat_steps.credentials.password.placeholder"),
                                error: None,
                                on_input: move |event: FormEvent| { password.set(event.value().clone()) },
                                on_keypress: move |event: KeyboardEvent| {
                                    if event.code() == keyboard_types::Code::Enter && ! username().is_empty() && ! password().is_empty() {
                                        auth.set_password(&password());
                                    }
                                },
                                on_click: move |_| {
                                    auth.set_password(&password());
                                }
                            }
                        }
                    )
                }
            } else if !flows.read().is_empty() {
                {
                    let f = flows.read();
                    let flows = f.clone();
                    let mut element = rsx!(div {});

                    for flow in flows.iter() {
                        element = match flow {
                            AuthType::ReCaptcha => rsx!(
                                div {
                                    onmounted: move |_| onloadCallback(),
                                    LoginForm {
                                        title: translate!(i18, "signup.chat_steps.captcha.title"),
                                        description: translate!(i18, "signup.chat_steps.captcha.description"),
                                        button_text: translate!(i18, "signup.chat_steps.captcha.cta"),
                                        emoji: "✍️",
                                        status: None,
                                        error: error(),
                                        on_handle: move |event: FormLoginEvent| {
                                            on_action_form(event, &mut on_handle_captcha)
                                        },
                                        body: rsx!(div {
                                            class: "signup__flow",
                                            id: "recaptcha-container",
                                        })
                                    }
                                }
                            ),
                            _ => rsx!(div {}),
                        };
                    }

                    element
                }
            } else {
                div { class: "column spinner-dual-ring--center", Spinner {} }
            }
        }
    )
}

pub enum SignupError {
    UnsupportedFlow,
    Unknown,
    KeyRecaptcha,
    Server,
    FlowNotFound,
    RegisterFailed,
    LoginFailed,
    SyncFailed,
    SessionFile,
    SetSiteKey,
}

fn flow_error(
    auth: &mut UseAuthState,
    homeserver: &mut Signal<String>,
    username: &mut Signal<String>,
    password: &mut Signal<String>,
    session_ref: &mut Signal<Option<String>>,
    f_error: &UiaaResponse,
    flows: &mut Signal<Vec<AuthType>>,
) -> Result<(), SignupError> {
    match f_error {
        uiaa::UiaaResponse::AuthResponse(uiaa_info) => {
            let completed = &uiaa_info.completed;
            let mut flows_to_complete: Vec<AuthType> = vec![];

            let flow = uiaa_info.flows.get(0).ok_or(SignupError::FlowNotFound)?;

            flow.stages
                .iter()
                .map(|f: &AuthType| -> Result<(), SignupError> {
                    if completed.iter().find(|e| *e == f).is_none() {
                        flows_to_complete.push(f.clone());
                    }
                    session_ref.set(uiaa_info.session.clone());

                    if let AuthType::ReCaptcha = f {
                        let params = uiaa_info.params.deref().get();
                        let uiaa_response =
                            serde_json::from_str(params).map_err(|_| SignupError::KeyRecaptcha)?;

                        set_site_key(uiaa_response)?;
                        flows.set(flows_to_complete.clone());

                        Ok(())
                    } else {
                        Err(SignupError::UnsupportedFlow)
                    }
                })
                .collect::<Result<(), SignupError>>()?;

            Ok(())
        }
        uiaa::UiaaResponse::MatrixError(_) => {
            reset_login_info(auth, homeserver, username, password);
            return Err(SignupError::Server);
        }
        _ => {
            reset_login_info(auth, homeserver, username, password);
            return Err(SignupError::Unknown);
        }
    }
}

fn reset_login_info(
    auth: &mut UseAuthState,
    homeserver: &mut Signal<String>,
    username: &mut Signal<String>,
    password: &mut Signal<String>,
) {
    homeserver.set(String::from(""));
    username.set(String::from(""));
    password.set(String::from(""));
    auth.reset();
}

fn set_site_key(uiaa_response: HashMap<&str, Value>) -> Result<(), SignupError> {
    let value = uiaa_response
        .get("m.login.recaptcha")
        .ok_or(SignupError::SetSiteKey)?;
    let Value::Object(ref recaptcha) = value else {
        return Err(SignupError::SetSiteKey);
    };

    let Some(Value::String(public_key)) = recaptcha.get("public_key") else {
        return Err(SignupError::SetSiteKey);
    };

    gloo::storage::LocalStorage::set("sitekey", public_key.clone())
        .map_err(|_| SignupError::SetSiteKey)
}
