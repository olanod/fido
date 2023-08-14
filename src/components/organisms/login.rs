use std::collections::HashMap;

use crate::components::atoms::message::{Message, Messages};
use crate::components::molecules::input_message::FormMessageEvent;
use crate::components::molecules::{InputMessage, List};
use crate::services::matrix::matrix::{build_client, login, FullSession, TimelineMessageType};
use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use futures_util::StreamExt;
use gloo::storage::errors::StorageError;
use gloo::storage::LocalStorage;
use log::info;
use matrix_sdk::config::SyncSettings;
use matrix_sdk::ruma::exports::serde_json;
use matrix_sdk::Client;

pub struct LoggedIn {
    pub is_logged_in: bool,
}

pub fn IndexLogin(cx: Scope) -> Element {
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

    let logged_in = use_shared_state::<LoggedIn>(cx).unwrap();

    let homeserver_login = use_ref::<String>(cx, || String::new());
    let username_login = use_ref::<String>(cx, || String::new());
    let next_id = use_ref(cx, || 1);

    let input_type = use_state::<String>(cx, || String::from("text"));
    let input_placeholder = use_state::<String>(cx, || {
        i18n_get_key_value(&i18n_map, "homeserver-placeholder")
    });
    let messages = use_state::<Messages>(cx, || {
        vec![Message {
            id: 0,
            display_name: i18n_get_key_value(&i18n_map, "actors-bot"),
            event_id: None,
            avatar_uri: None,
            content: TimelineMessageType::Text(i18n_get_key_value(&i18n_map, "homeserver-message")),
            reply: None,
        }]
    });

    let login_task = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![
            homeserver_login,
            username_login,
            messages,
            next_id,
            input_type,
            input_placeholder,
            logged_in
        ];

        async move {
            while let Some(message_item) = rx.next().await {
                if input_type.current().to_string().eq("password") {
                    let hint_message: String = message_item.chars().map(|_| '*').collect();
                    push_message(
                        TimelineMessageType::Text(hint_message),
                        next_id.to_owned(),
                        i18n_get_key_value(&i18n_map, "actors-user"),
                        messages.to_owned(),
                    );
                } else {
                    push_message(
                        TimelineMessageType::Text(message_item.clone()),
                        next_id.to_owned(),
                        i18n_get_key_value(&i18n_map, "actors-user"),
                        messages.to_owned(),
                    );
                }

                input_type.set(String::from("text"));
                input_placeholder.set(i18n_get_key_value(&i18n_map, "homeserver-placeholder"));

                if homeserver_login.read().len() == 0 {
                    let result = build_client(message_item.clone()).await;

                    match result {
                        Ok(_) => {
                            push_message(
                                TimelineMessageType::Text(i18n_get_key_value(
                                    &i18n_map,
                                    "username-message",
                                )),
                                next_id.to_owned(),
                                i18n_get_key_value(&i18n_map, "actors-bot"),
                                messages.to_owned(),
                            );

                            homeserver_login.set(message_item.clone());
                            input_placeholder
                                .set(i18n_get_key_value(&i18n_map, "username-placeholder"));
                        }
                        Err(err) => {
                            info!("homeserver error: {err}");

                            if err.to_string().eq("relative URL without a base") {
                                push_message(
                                    TimelineMessageType::Text(i18n_get_key_value(
                                        &i18n_map,
                                        "invalid-url",
                                    )),
                                    next_id.to_owned(),
                                    i18n_get_key_value(&i18n_map, "actors-bot"),
                                    messages.to_owned(),
                                );
                            } else {
                                push_message(
                                    TimelineMessageType::Text(i18n_get_key_value(
                                        &i18n_map, "unknown",
                                    )),
                                    next_id.to_owned(),
                                    i18n_get_key_value(&i18n_map, "actors-bot"),
                                    messages.to_owned(),
                                );
                            }

                            push_message(
                                TimelineMessageType::Text(i18n_get_key_value(
                                    &i18n_map,
                                    "homeserver-message",
                                )),
                                next_id.to_owned(),
                                i18n_get_key_value(&i18n_map, "actors-bot"),
                                messages.to_owned(),
                            );
                        }
                    }
                } else if username_login.read().len() == 0 {
                    input_type.set(String::from("password"));

                    push_message(
                        TimelineMessageType::Text(i18n_get_key_value(
                            &i18n_map,
                            "password-message",
                        )),
                        next_id.to_owned(),
                        i18n_get_key_value(&i18n_map, "actors-bot"),
                        messages.to_owned(),
                    );

                    username_login.set(message_item.clone());
                    input_placeholder.set(i18n_get_key_value(&i18n_map, "password-placeholder"));
                } else {
                    push_message(
                        TimelineMessageType::Text(i18n_get_key_value(
                            &i18n_map,
                            "messages-validating",
                        )),
                        next_id.to_owned(),
                        i18n_get_key_value(&i18n_map, "actors-bot"),
                        messages.to_owned(),
                    );

                    let user = username_login.read().clone();
                    let server = homeserver_login.read().clone();

                    let response = login(
                        String::from(server),
                        String::from(user),
                        String::from(message_item.clone()),
                    )
                    .await;

                    match response {
                        Ok((client, serialized_session)) => {
                            push_message(
                                TimelineMessageType::Text(i18n_get_key_value(
                                    &i18n_map,
                                    "messages-welcome",
                                )),
                                next_id.to_owned(),
                                i18n_get_key_value(&i18n_map, "actors-bot"),
                                messages.to_owned(),
                            );

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
                                push_message(
                                    TimelineMessageType::Text(i18n_get_key_value(
                                        &i18n_map,
                                        "invalid_username_password",
                                    )),
                                    next_id.to_owned(),
                                    i18n_get_key_value(&i18n_map, "actors-bot"),
                                    messages.to_owned(),
                                );
                            } else {
                                push_message(
                                    TimelineMessageType::Text(i18n_get_key_value(
                                        &i18n_map, "unknown",
                                    )),
                                    next_id.to_owned(),
                                    i18n_get_key_value(&i18n_map, "actors-bot"),
                                    messages.to_owned(),
                                );
                            }

                            push_message(
                                TimelineMessageType::Text(i18n_get_key_value(
                                    &i18n_map,
                                    "homeserver-message",
                                )),
                                next_id.to_owned(),
                                i18n_get_key_value(&i18n_map, "actors-bot"),
                                messages.to_owned(),
                            );

                            homeserver_login.set(String::new());
                            username_login.set(String::new());
                        }
                    }
                }
            }
        }
    });

    let search = move |evt: FormMessageEvent| {
        login_task.send(evt.value);
    };

    cx.render(rsx! {
        div {
            class:"chat",
            List {
                messages: messages
            }
            InputMessage {
                message_type: input_type.get().as_str(),
                replying_to: &None,
                placeholder: input_placeholder.get().as_str(),
                is_attachable: false,
                on_submit: search
                on_event: move |_| {}
            }
        }
    })
}

pub fn push_message(
    content: TimelineMessageType,
    next_id: UseRef<i64>,
    display_name: String,
    messages: UseState<Vec<Message>>,
) {
    messages.with_mut(|m| {
        m.push(Message {
            id: *next_id.read(),
            display_name: display_name,
            event_id: None,
            avatar_uri: None,
            content: content,
            reply: None,
        });

        m.rotate_right(1)
    });

    let current_id = *next_id.read();
    next_id.set(current_id + 1);
}

pub fn i18n_get_key_value(i18n_map: &HashMap<&str, String>, key: &str) -> String {
    i18n_map.get_key_value(key).unwrap().1.clone()
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
