use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::{
        atoms::{input::InputType, MessageInput},
        organisms::{login_form::FormLoginEvent, LoginForm},
    },
    hooks::{
        use_auth::{use_auth, CacheLogin},
        use_client::use_client,
        use_init_app::BeforeSession,
        use_notification::use_notification,
        use_session::use_session,
    },
    services::matrix::matrix::login,
};

#[derive(Debug, Clone)]
pub struct LoggedIn(pub bool);

#[derive(PartialEq)]
pub enum LoggedInStatus {
    Start,
    Loading,
    Done,
    Persisting,
    LoggedAs(String),
}

impl LoggedInStatus {
    fn has_status(&self) -> bool {
        match self {
            LoggedInStatus::Start
            | LoggedInStatus::Loading
            | LoggedInStatus::Done
            | LoggedInStatus::Persisting
            | LoggedInStatus::LoggedAs(_) => true,
        }
    }

    fn get_text(
        &self,
        key_loading: &str,
        key_logged: &str,
        key_done: &str,
        key_persisting: &str,
    ) -> Option<String> {
        match self {
            LoggedInStatus::Loading => Some(key_loading.to_owned()),
            LoggedInStatus::LoggedAs(_) => Some(key_logged.to_owned()),
            LoggedInStatus::Done => Some(key_done.to_owned()),
            LoggedInStatus::Persisting => Some(key_persisting.to_owned()),
            LoggedInStatus::Start => None,
        }
    }
}

enum LoginFrom {
    SavedData,
    FullForm,
}

pub fn Login() -> Element {
    let i18 = use_i18();

    let mut client = use_client();
    let mut auth = use_auth();
    let mut session = use_session();
    let mut notification = use_notification();

    let mut homeserver = use_signal(|| String::from(""));
    let mut username = use_signal(|| String::from(""));
    let mut password = use_signal(|| String::from(""));
    let mut error = use_signal(|| None);
    let mut login_from = use_signal(|| {
        if auth.is_storage_data() {
            LoginFrom::SavedData
        } else {
            LoginFrom::FullForm
        }
    });
    let mut is_loading_loggedin = use_signal::<LoggedInStatus>(|| LoggedInStatus::Start);

    let mut before_session = consume_context::<Signal<BeforeSession>>();

    let on_handle_clear = move || {
        spawn({
            async move {
                auth.reset();
                login_from.set(LoginFrom::FullForm);

                username.set(String::new());
                password.set(String::new());
            }
        });
    };

    let on_handle_login = move || {
        spawn({
            async move {
                is_loading_loggedin.set(LoggedInStatus::Loading);
                if username.read().contains(':') {
                    let username = username();
                    let parts = username.splitn(2, ':').collect::<Vec<&str>>();

                    if let Err(_) = auth.set_server(parts[1]).await {
                        notification.handle_error(&format!(
                            "{}: {}",
                            translate!(i18, "login.chat_errors.invalid_server"),
                            parts[1]
                        ));
                        is_loading_loggedin.set(LoggedInStatus::Start);
                        return;
                    };
                } else {
                    if let Err(e) = auth.set_server(&homeserver()).await {
                        log::warn!("Failed to set server: {e:?}");
                    }
                }

                auth.set_username(&username(), true);
                auth.set_password(&password());

                let login_config = auth.build();

                let Ok(info) = login_config else {
                    username.set(String::new());
                    password.set(String::new());

                    return auth.reset();
                };
                let response =
                    login(&info.server.to_string(), &info.username, &info.password).await;

                match response {
                    Ok((c, serialized_session)) => {
                        is_loading_loggedin.set(LoggedInStatus::Done);

                        let display_name = c.account().get_display_name().await.ok().flatten();

                        if let Err(_) = session.persist_session_file(&serialized_session) {
                            notification
                                .handle_error(&translate!(i18, "chat.common.error.persist"));
                        };

                        is_loading_loggedin.set(LoggedInStatus::Persisting);

                        if let Err(_) = session.sync(c.clone(), None).await {
                            notification.handle_error(&translate!(i18, "chat.common.error.sync"));
                        };

                        client.set(crate::MatrixClientState {
                            client: Some(c.clone()),
                        });

                        if let Err(_) = auth.persist_data(CacheLogin {
                            server: homeserver(),
                            username: username(),
                            display_name,
                        }) {
                            notification
                                .handle_error(&translate!(i18, "chat.common.error.persist"));
                        };
                        auth.set_logged_in(true);
                    }
                    Err(err) => {
                        is_loading_loggedin.set(LoggedInStatus::Start);
                        if err
                            .to_string()
                            .eq("the server returned an error: [403 / M_FORBIDDEN] Invalid username or password")
                        {
                            error.set(Some(translate!(i18, "login.chat_errors.invalid_username_password")))
                        } else {
                            error.set(Some(translate!(i18, "login.chat_errors.unknown")))
                        }

                        username.set(String::new());
                        password.set(String::new());

                        auth.reset();
                    }
                }
            }
        });
    };

    use_coroutine(|_: UnboundedReceiver<()>| async move {
        let Ok(data) = auth.get_storage_data() else {
            let url = client.get().homeserver().await;
            let Some(domain) = url.domain() else {
                return;
            };
            return homeserver.set(format!("{}://{}", url.scheme(), domain));
        };

        let deserialize_data = serde_json::from_str::<CacheLogin>(&data);

        if let Ok(data) = deserialize_data {
            auth.set_login_cache(data.clone());

            homeserver.set(data.server.clone());
            username.set(data.username.clone());

            if let Err(e) = auth.set_server(&homeserver()).await {
                log::warn!("Failed to set server: {e:?}");
            }
            auth.set_username(&data.username, true);
        }
    });

    let on_handle_form_event = move |event: FormLoginEvent| match event {
        FormLoginEvent::FilledForm => on_handle_login(),
        FormLoginEvent::Login => *before_session.write() = BeforeSession::Login,
        FormLoginEvent::CreateAccount => *before_session.write() = BeforeSession::Signup,
        FormLoginEvent::Guest => *before_session.write() = BeforeSession::Guest,
        FormLoginEvent::ClearData => on_handle_clear(),
    };

    rsx!(
        div { class: "page--clamp",
            if (auth.is_storage_data()&& matches!(*is_loading_loggedin.read(), LoggedInStatus::Start))
                || (is_loading_loggedin.read().has_status() && matches!(*login_from.read(), LoginFrom::SavedData)) {
                {
                    let display_name = auth.get_login_cache().map(|data| data.display_name.unwrap_or(data.username)).unwrap_or(String::from(""));

                    let loggedin_status = is_loading_loggedin.read().get_text(
                        &translate!(i18, "login.status.loading"),
                        &translate!(i18, "login.status.logged"),
                        &translate!(i18, "login.status.done"),
                        &translate!(i18, "login.status.persisting")
                    );
                    rsx!(
                        LoginForm {
                            title: r#"{translate!(i18, "login.unlock.title")} {display_name}"#,
                            description: translate!(i18, "login.unlock.description"),
                            button_text: translate!(i18, "login.unlock.cta"),
                            emoji: "👋",
                            error: error(),
                            clear_data: true,
                            status: loggedin_status.map(|t|String::from(t)),
                            on_handle: on_handle_form_event,
                            body: rsx!(
                                div {
                                    MessageInput {
                                        itype: InputType::Password,
                                        message: "{password()}",
                                        placeholder: translate!(i18, "login.chat_steps.credentials.password.placeholder"),
                                        error: None,
                                        on_input: move |event: FormEvent| {
                                            password.set(event.value().clone())
                                        },
                                        on_keypress: move |event: KeyboardEvent| {
                                            if event.code() == keyboard_types::Code::Enter && !password().is_empty() {
                                                on_handle_login();
                                            }
                                        },
                                        on_click: move |_| {
                                            auth.set_password(&password())
                                        }
                                    }
                                }
                            )
                        }
                    )
                }
            } else if (auth.get().data.username.is_none() || auth.get().data.password.is_none())
                || (is_loading_loggedin.read().has_status() && matches!(*login_from.read(), LoginFrom::FullForm)) {
                {
                    let loggedin_status = is_loading_loggedin.read().get_text(
                        &translate!(i18, "login.status.loading"),
                        &translate!(i18, "login.status.logged"),
                        &translate!(i18, "login.status.done"),
                        &translate!(i18, "login.status.persisting")
                    );
                    rsx!(
                        LoginForm {
                            title: translate!(i18, "login.chat_steps.credentials.title"),
                            description: translate!(i18, "login.chat_steps.credentials.description"),
                            button_text: translate!(i18, "login.chat_steps.credentials.cta"),
                            emoji: "👋",
                            error: error(),
                            on_handle: on_handle_form_event,
                            status: loggedin_status.map(String::from),
                            body: rsx!(
                                div {
                                    MessageInput {
                                        message: "{username()}",
                                        placeholder: translate!(i18, "login.chat_steps.credentials.username.placeholder"),
                                        error: None,
                                        on_input: move |event: FormEvent| {
                                            username.set(event.value().clone())
                                        },
                                        on_keypress: move |event: KeyboardEvent| {
                                            if event.code() == keyboard_types::Code::Enter && !username().is_empty() {
                                                auth.set_username(&username(), true)
                                            }
                                        },
                                        on_click: move |_| {
                                            auth.set_username(&username(), true)
                                        }
                                    }
                                }
                                div {
                                    MessageInput {
                                        itype: InputType::Password,
                                        message: "{password()}",
                                        placeholder: translate!(i18, "login.chat_steps.credentials.password.placeholder"),
                                        error: None,
                                        on_input: move |event: FormEvent| {
                                            password.set(event.value().clone())
                                        },
                                        on_keypress: move |event: KeyboardEvent| {
                                            if event.code() == keyboard_types::Code::Enter && !username().is_empty() && !password().is_empty() {
                                                on_handle_login();
                                            }
                                        },
                                        on_click: move |_| {
                                            auth.set_password(&password());
                                        }
                                    }
                                }
                            )
                        }
                    )
                }
            }
        }
    )
}
