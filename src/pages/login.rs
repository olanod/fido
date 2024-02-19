use std::{collections::HashMap, rc::Rc};

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{translate, i18n::use_i18};
use gloo::storage::LocalStorage;
use crate::{
    components::{
        atoms::{MessageInput, input::InputType},
        organisms::{login_form::FormLoginEvent, LoginForm},
    },
    utils::i18n_get_key_value::i18n_get_key_value, services::matrix::matrix::login, hooks::{use_client::use_client, use_init_app::BeforeSession, use_auth::{use_auth, CacheLogin}, use_session::use_session, use_notification::use_notification},
};

#[derive(Debug, Clone)]
pub struct LoggedIn(pub bool);

#[derive(PartialEq)]
pub enum LoggedInStatus {
    Start,
    Loading,
    Done,
    Persisting,
    LoggedAs(String)
}

impl LoggedInStatus {
    fn has_status(&self) -> bool {
        match self {
            LoggedInStatus::Start |
            LoggedInStatus::Loading |
            LoggedInStatus::Done |
            LoggedInStatus::Persisting |
            LoggedInStatus::LoggedAs(_) => true,
            _ => false
        }
    } 

    fn get_text<'a>(&self, key_loading: &'a str,  key_logged: &'a str,  key_done: &'a str, key_persisting: &'a str) -> Option<&'a str> {
        match self {
            LoggedInStatus::Loading => Some(key_loading),
            LoggedInStatus::LoggedAs(_) => Some(key_logged),
            LoggedInStatus::Done => Some(key_done),
            LoggedInStatus::Persisting => Some(key_persisting),
            LoggedInStatus::Start => None
        }
    }
}

enum LoginFrom {
    SavedData,
    FullForm
}

pub fn Login(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    let key_chat_errors_not_found = translate!(i18, "login.chat_errors.not_found");
    let key_login_unlock_title = translate!(i18, "login.unlock.title");
    let key_login_unlock_description = translate!(i18, "login.unlock.description");
    let key_login_unlock_cta = translate!(i18, "login.unlock.cta");

    let key_login_chat_homeserver_message = "login-chat-homeserver-message";
    let key_login_chat_homeserver_description = "login-chat-homeserver-description";
    let key_login_chat_homeserver_placeholder = "login-chat-homeserver-placeholder";
    let key_login_chat_homeserver_cta = "login-chat-homeserver-cta";

    let key_login_chat_credentials_description = "login-chat-credentials-description";
    let key_login_chat_credentials_title = "login-chat-credentials-title";

    let key_login_chat_credentials_username_message = "login-chat-credentials-username-message";
    let key_login_chat_credentials_username_placeholder = "login-chat-credentials-username-placeholder";

    let key_login_chat_credentials_password_message = "login-chat-credentials-password-message";
    let key_login_chat_credentials_password_placeholder = "login-chat-credentials-password-placeholder";

    let key_login_chat_credentials_cta = "login-chat-credentials-cta";

    let key_login_chat_messages_validating = "login-chat-messages-validating";
    let key_login_chat_messages_welcome = "login-chat-messages-welcome";

    let key_login_chat_errors_invalid_url = "login-chat-errors-invalid-url";
    let key_login_chat_errors_unknown = "login-chat-errors-unknown";
    let key_login_chat_errors_invalid_username_password = "login-chat-errors-invalid-username-password";

    let key_login_status_loading = translate!(i18, "login.status.loading");
    let key_login_status_logged = translate!(i18, "login.status.logged");
    let key_login_status_done = translate!(i18, "login.status.done");
    let key_login_status_persisting = translate!(i18, "login.status.persisting");

    let i18n_map = HashMap::from([
        (key_login_chat_homeserver_message, translate!(i18, "login.chat_steps.homeserver.message")),
        (key_login_chat_homeserver_description, translate!(i18, "login.chat_steps.homeserver.description")),
        (key_login_chat_homeserver_placeholder, translate!(i18, "login.chat_steps.homeserver.placeholder")),
        (key_login_chat_homeserver_cta, translate!(i18, "login.chat_steps.homeserver.cta")),

        (key_login_chat_credentials_title, translate!(i18, "login.chat_steps.credentials.title")),
        
        (key_login_chat_credentials_description, translate!(i18, "login.chat_steps.credentials.description")),

        (key_login_chat_credentials_username_message, translate!(i18, "login.chat_steps.credentials.username.message")),
        (key_login_chat_credentials_username_placeholder, translate!(i18, "login.chat_steps.credentials.username.placeholder")),

        (key_login_chat_credentials_password_message, translate!(i18, "login.chat_steps.credentials.password.message")),
        (key_login_chat_credentials_password_placeholder, translate!(i18, "login.chat_steps.credentials.password.placeholder")),
        (key_login_chat_credentials_cta, translate!(i18, "login.chat_steps.credentials.cta")),

        (key_login_chat_messages_validating, translate!(i18, "login.chat_steps.messages.validating")),
        (key_login_chat_messages_welcome, translate!(i18, "login.chat_steps.messages.welcome")),

        (key_login_chat_errors_invalid_url, translate!(i18, "login.chat_errors.invalid_url")),
        (key_login_chat_errors_unknown, translate!(i18, "login.chat_errors.unknown")),
        (key_login_chat_errors_invalid_username_password, translate!(i18, "login.chat_errors.invalid_username_password")),
    ]);

    let client = use_client(cx);
    let auth = use_auth(cx);
    let session = use_session(cx);
    let notification = use_notification(cx);

    let homeserver = use_state(cx, || String::from(""));
    let username = use_state(cx, || String::from(""));
    let password = use_state(cx, || String::from(""));
    let error = use_state(cx, || None);
    let login_from = use_state(cx, || if auth.is_storage_data() {LoginFrom::SavedData} else {LoginFrom::FullForm});

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
                auth.set_server(homeserver.get()).await;
            }
        })
    };

    let on_handle_clear = Rc::new(move || {
        cx.spawn({
            to_owned![homeserver, username, password, auth, login_from];

            async move {
                auth.reset();
                login_from.set(LoginFrom::FullForm);

                homeserver.set(String::new());
                username.set(String::new());
                password.set(String::new());
            }
        })
    });

    let on_handle_clear_clone = on_handle_clear.clone();

    let on_handle_login = Rc::new(move || {
        auth.set_server(homeserver.get());
        auth.set_username(username.get(), true);
        auth.set_password(password.get());

        cx.spawn({
            to_owned![auth, session, username, password, is_loading_loggedin, client, error, error_invalid_credentials, error_unknown, homeserver, notification];

            async move {
                is_loading_loggedin.set(LoggedInStatus::Loading);
                let login_config = auth.build();

                let Ok(info) = login_config else  {
                    homeserver.set(String::new());
                    username.set(String::new());
                    password.set(String::new());
                    
                    return auth.reset();
                };

                let response = login(
                    &info.server.to_string(),
                    &info.username,
                    &info.password,
                )
                .await;

                match response {
                    Ok((c, serialized_session)) => {
                        is_loading_loggedin.set(LoggedInStatus::Done);

                        let display_name = c.account().get_display_name().await.ok().flatten();

                        <LocalStorage as gloo::storage::Storage>::set(
                            "session_file",
                            serialized_session,
                        );

                        is_loading_loggedin.set(LoggedInStatus::Persisting);
        
                        session.sync(c.clone(), None).await;

                        client.set(crate::MatrixClientState { client: Some(c.clone()) });
                        
                        let Some(user_id) = c.user_id() else {
                            return notification.handle_error("{key_chat_errors_not_found}");
                        };

                        is_loading_loggedin.set(LoggedInStatus::LoggedAs(user_id.to_string()));

                        auth.set_logged_in(true);

                        auth.persist_data(CacheLogin {
                            server: homeserver.get().to_string(),
                            username: username.get().to_string(),
                            display_name
                        });

                    }
                    Err(err) => {
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
        })
    });

    let on_handle_login_key_press = on_handle_login.clone();
    let on_handle_login_clone = on_handle_login.clone();

    use_coroutine(cx, |_: UnboundedReceiver::<()>| {
        to_owned![auth, homeserver, username];
        
        async move {
            let data = auth.get_storage_data();

            if let Ok(data) = data {
                let deserialize_data = serde_json::from_str::<CacheLogin>(&data);

                if let Ok(data) = deserialize_data {
                    auth.set_login_cache(data.clone());

                    homeserver.set(data.server.clone());
                    username.set(data.username.clone());
                    
                    auth.set_server(&data.server).await;
                    auth.set_username(&data.username, true);
                }
            }
        }
    });

    let on_handle_form_event = move |event: FormLoginEvent| match event {
        FormLoginEvent::FilledForm => on_handle_login_clone(),
        FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
        FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
        FormLoginEvent::ClearData => on_handle_clear_clone()
    };

    render!(
        div {
            class: "page--clamp",
            if (auth.is_storage_data() && matches!(*is_loading_loggedin.read(), LoggedInStatus::Start)) || (is_loading_loggedin.read().has_status() && matches!(*login_from.get(), LoginFrom::SavedData)) {
                let display_name = auth.get_login_cache().map(|data| data.display_name.unwrap_or(data.username)).unwrap_or(String::from(""));
                
                let loggedin_status = is_loading_loggedin.read().get_text(
                    &key_login_status_loading,
                    &key_login_status_logged,
                    &key_login_status_done,
                    &key_login_status_persisting
                );

                rsx!(
                    LoginForm {
                        title: "{key_login_unlock_title} {display_name}",
                        description: "{key_login_unlock_description}",
                        button_text: "{key_login_unlock_cta}",
                        emoji: "👋",
                        error: error.get().as_ref(),
                        clear_data: true,
                        status: loggedin_status.map(|t|String::from(t)),
                        on_handle: on_handle_form_event,
                        body: render!(rsx!(
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
                                        if event.code() == keyboard_types::Code::Enter && !password.get().is_empty() {
                                            on_handle_login_key_press()
                                        }
                                    },
                                    on_click: move |_| {
                                        auth.set_password(password.get())
                                    }
                                }
                            }
                        ))
                    }
                )
            } else if auth.get().data.server.is_none() {
                let on_handle_login = on_handle_login.clone();
                rsx!(
                    LoginForm {
                        title: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_message)}",
                        description: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_description)}",
                        button_text: "{i18n_get_key_value(&i18n_map, key_login_chat_homeserver_cta)}",
                        emoji: "🛰️",
                        error: error.get().as_ref(),
                        on_handle: move |event: FormLoginEvent| match event {
                            FormLoginEvent::FilledForm => on_update_homeserver(),
                            FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
                            FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
                            FormLoginEvent::ClearData => on_handle_clear()
                        },
                        status: None,
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
                                        if event.code() == keyboard_types::Code::Enter && !homeserver.get().is_empty() {
                                            on_update_homeserver()
                                        }
                                    },
                                    on_click: move |_| {
                                        on_update_homeserver()
                                    }
                                }
                            },
                        
                        ))
                    }
                )
            } else if (auth.get().data.username.is_none() || auth.get().data.password.is_none()) || (is_loading_loggedin.read().has_status() && matches!(*login_from.get(), LoginFrom::FullForm)) {
                let loggedin_status = is_loading_loggedin.read().get_text(
                    &key_login_status_loading,
                    &key_login_status_logged,
                    &key_login_status_done,
                    &key_login_status_persisting
                );

                rsx!(
                    LoginForm {
                        title: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_title)}",
                        description: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_description)}",
                        button_text: "{i18n_get_key_value(&i18n_map, key_login_chat_credentials_cta)}",
                        emoji: "👋",
                        error: error.get().as_ref(),
                        on_handle: on_handle_form_event,
                        status: loggedin_status.map(|t|String::from(t)),
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
                                            auth.set_username(username.get(), true)
                                        }
                                    },
                                    on_click: move |_| {
                                        auth.set_username(username.get(), true)
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
                                            on_handle_login_key_press()
                                        }
                                    },
                                    on_click: move |_| {
                                        auth.set_password(password.get());
                                    }
                                }
                            }
                        ))
                    }
                )
            }
        }
    )
}
