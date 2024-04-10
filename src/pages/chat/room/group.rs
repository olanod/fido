use std::{collections::HashMap, ops::Deref};

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};
use matrix_sdk::ruma::{OwnedUserId, UserId};

use crate::{
    components::{
        atoms::{
            attach::AttachType, button::Variant, Attach, Avatar, Button, Close, Header, Icon,
            MessageInput, RoomView,
        },
        molecules::{rooms::CurrentRoom, Guest},
    },
    hooks::{
        use_attach::{use_attach, AttachError, AttachFile},
        use_client::use_client,
        use_notification::use_notification,
        use_room::use_room,
        use_session::use_session,
    },
    pages::chat::room::new::CreationStatus,
    services::matrix::matrix::{create_room, find_user_by_id},
    utils::{
        i18n_get_key_value::i18n_get_key_value,
        matrix::{mxc_to_thumbnail_uri, ImageMethod, ImageSize},
        sync_room::sync_created_room,
    },
};
use futures_util::{StreamExt, TryFutureExt};

#[derive(Clone, Debug)]
pub struct Profile {
    pub displayname: String,
    pub avatar_uri: Option<String>,
    pub id: String,
}

pub struct SelectedProfiles {
    profiles: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum CreateRoomError {
    InvalidUserId,
    UserNotFound,
    InvalidUsername,
    ServerError,
}

pub fn RoomGroup() -> Element {
    use_context_provider::<Signal<SelectedProfiles>>(|| {
        Signal::new(SelectedProfiles { profiles: vec![] })
    });
    use_context_provider::<Signal<Option<AttachFile>>>(|| Signal::new(None));

    let i18 = use_i18();

    let navigation = use_navigator();
    let client = use_client();
    let mut attach = use_attach();
    let mut notification = use_notification();
    let mut room = use_room();
    let session = use_session();

    let mut selected_users = consume_context::<Signal<SelectedProfiles>>();

    let mut user_id = use_signal::<String>(|| String::from(""));
    let mut users = use_signal::<Vec<Profile>>(|| vec![]);

    let error = use_signal::<Option<String>>(|| None);

    let mut handle_complete_group = use_signal::<bool>(|| false);
    let mut group_name = use_signal::<String>(|| String::from(""));
    let mut status = use_signal::<CreationStatus>(|| CreationStatus::Start);

    let task_search_user = use_coroutine(|mut rx: UnboundedReceiver<String>| async move {
        while let Some(id) = rx.next().await {
            let element = users.read().clone().into_iter().find(|u| u.id.eq(&id));

            if let None = element {
                match find_user_by_id(&id, &client.get()).await {
                    Ok(profile) => users.with_mut(|user| user.push(profile)),
                    Err(_) => {
                        notification.handle_error(&translate!(i18, "group.error.not_found"));
                    }
                }
            }
        }
    });

    let on_handle_create = move |_| {
        spawn({
            async move {
                status.set(CreationStatus::Creating);
                let users = selected_users
                    .read()
                    .profiles
                    .clone()
                    .into_iter()
                    .map(|p| UserId::parse(p).expect("Unable to read user profile"))
                    .collect::<Vec<OwnedUserId>>();

                let avatar = attach.get().map(|file| file.data);
                let name = group_name();

                let room_meta =
                    create_room(&client.get(), false, &users, Some(name.clone()), avatar)
                        .await
                        .map_err(|_| CreateRoomError::ServerError)?;

                status.set(CreationStatus::Ok);

                sync_created_room(&room_meta.room_id, &client.get()).await;

                let room_info = client
                    .get()
                    .get_room(&room_meta.room_id)
                    .expect("Unable to load created room");

                room.set(CurrentRoom {
                    id: room_meta.room_id.to_string().clone(),
                    name: name,
                    avatar_uri: room_info
                        .avatar_url()
                        .map(|uri| {
                            mxc_to_thumbnail_uri(&uri, ImageSize::default(), ImageMethod::CROP)
                        })
                        .flatten(),
                });

                navigation.go_back();

                Ok::<(), CreateRoomError>(())
            }
            .unwrap_or_else(move |e: CreateRoomError| {
                let message_error = match e {
                    CreateRoomError::InvalidUserId => translate!(i18, "chat.common.error.user_id"),
                    CreateRoomError::UserNotFound => translate!(i18, "chat.common.error.server"),
                    CreateRoomError::InvalidUsername => translate!(i18, "group.error.not_found"),
                    CreateRoomError::ServerError => translate!(i18, "group.error.profile"),
                };

                status.set(CreationStatus::Error(e));
                notification.handle_error(&message_error);
            })
        });
    };

    let on_handle_attach = move |event: Event<FormData>| {
        spawn({
            async move {
                let files = &event.files().ok_or(AttachError::NotFound)?;
                let fs = files.files();

                let existing_file = fs.get(0).ok_or(AttachError::NotFound)?;
                let content = files
                    .read_file(existing_file)
                    .await
                    .ok_or(AttachError::NotFound)?;
                let infered_type = infer::get(content.deref()).ok_or(AttachError::UncoverType)?;

                let content_type: Result<mime::Mime, _> = infered_type.mime_type().parse();
                let content_type = content_type.map_err(|_| AttachError::UnknownContent)?;

                let blob = match content_type.type_() {
                    mime::IMAGE => gloo::file::Blob::new(content.deref()),
                    mime::VIDEO => gloo::file::Blob::new_with_options(
                        content.deref(),
                        Some(infered_type.mime_type()),
                    ),
                    _ => gloo::file::Blob::new(content.deref()),
                };

                let size = blob.size().clone();
                let object_url = gloo::file::ObjectUrl::from(blob);

                attach.set(Some(AttachFile {
                    name: existing_file.to_string(),
                    preview_url: object_url,
                    data: content.clone(),
                    content_type,
                    size,
                }));

                Ok::<(), AttachError>(())
            }
            .unwrap_or_else(move |e: AttachError| {
                let message_error = match e {
                    AttachError::NotFound => translate!(i18, "chat.input_message.not_found"),
                    AttachError::UncoverType => translate!(i18, "chat.input_message.file_type"),
                    AttachError::UnknownContent => translate!(i18, "chat.input_message.not_found"),
                };

                notification.handle_error(&message_error);
            })
        });
    };

    let element = if let Ok(file) = attach.get_file() {
        rsx!( img { class: "group__attach", src: "{file.deref()}" } )
    } else {
        rsx!(
            Avatar {
                name: if !group_name().is_empty() { String::from(group_name()) } else { String::from("X") },
                size: 80,
                uri: None
            }
        )
    };

    rsx! {
        Header {
            text: translate!(i18, "group.title"),
            on_event: move |_| { navigation.go_back() }
        }
        if *handle_complete_group.read() {
            Attach { atype: AttachType::Avatar(element), on_click: on_handle_attach }

            MessageInput {
                message: "{group_name()}",
                placeholder: translate!(i18, "group.meta.placeholder"),
                label: translate!(i18, "group.meta.label"),
                error: error(),
                on_input: move |event: Event<FormData>| {
                    group_name.set(event.value());
                },
                on_keypress: move |_| {},
                on_click: move |_| {}
            }
            p { class: "group__title",
                {translate!(i18, "group.meta.members.title")}
            }
            {
                users.read().deref().into_iter().map(|u| {
                    selected_users.read().profiles.clone().into_iter().position(|selected_p| selected_p.eq(&u.id)).map(|position| {
                        rsx!(
                            div {
                                class: "group__users",
                                RoomView {
                                    displayname: "{u.displayname.clone()}",
                                    avatar_uri: None,
                                    description: "",
                                    on_click: move |_| {}
                                }
                                button {
                                    class: "group__cta--close",
                                    onclick: move |_| {
                                        selected_users.write().profiles.remove(position);
                                    },
                                    Icon {
                                        stroke: "var(--icon-subdued)",
                                        icon: Close
                                    }
                                }
                            }
                        )
                    }).flatten()
                })
            }
            if !matches!(*status.read(), CreationStatus::Start) {
                match *status.read() {
                    CreationStatus::Creating => {
                        rsx! {
                            div {
                                class: "room-new__status-container",
                                p {
                                    class: "room-new__status__description",
                                    {translate!(i18, "group.status.creating")}
                                }
                            }
                        }
                    },
                    CreationStatus::Ok => {
                        rsx! {
                            div {
                                class: "room-new__status-container",
                                p {
                                    class: "room-new__status__description",
                                    {translate!(i18, "group.status.created")}
                                }
                            }
                        }
                    },
                    CreationStatus::Error(CreateRoomError::ServerError) => {
                        let cta_back = translate!(i18, "group.status.error.cta.back");
                        let cta_try = translate!(i18, "group.status.error.cta.try");
                        rsx! {
                            div {
                                class: "room-new__status-container",
                                h3 {
                                    class: "room-new__status__title",
                                    {translate!(i18, "group.status.error.title")}
                                }
                                p {
                                    class: "room-new__status__description",
                                    {translate!(i18, "group.status.error.description")}
                                }
                                div {
                                    class: "row room-new__status-cta",
                                    Button{
                                        text: "{cta_back}",
                                        variant: Variant::Secondary,
                                        on_click: move |_| {
                                            navigation.go_back()
                                        },
                                        status: None
                                    }
                                    Button{
                                        text: "{cta_try}",
                                        on_click: on_handle_create,
                                        status: None
                                    }
                                }
                            }
                        }
                    },
                    _ => None
                }
            } else {
                if session.is_guest() {
                    Guest {
                        description: translate!(i18, "chat.guest.signup.description"),
                        cta: translate!(i18, "chat.guest.signup.cta"),
                        on_click: move |_| {}
                    }
                } else {
                    div { class: "group__cta__wrapper row",
                        Button {
                            text: translate!(i18, "group.meta.cta.back"),
                            status: None,
                            variant: Variant::Secondary,
                            on_click: move |_| {}
                        }
                        Button {
                            text: translate!(i18, "group.meta.cta.create"),
                            status: None,
                            disabled: group_name().is_empty(),
                            on_click: on_handle_create
                        }
                    }
                }
            }
        } else {

            MessageInput {
                message: "{user_id}",
                placeholder: translate!(i18, "group.select.placeholder"),
                label: translate!(i18, "group.select.label"),
                error: error(),
                on_input: move |event: Event<FormData>| {
                    user_id.set(event.value().clone());
                },
                on_keypress: move |event: KeyboardEvent| {
                    if event.code() == keyboard_types::Code::Enter && !user_id().is_empty() {
                        task_search_user.send(user_id())
                    }
                },
                on_click: move |_| { task_search_user.send(user_id()) }
            }
            form {
                class: "group__form",
                onchange: move |event| {
                    let values = event.values().clone().into_keys().collect::<Vec<String>>();
                    let profiles = selected_users.read().deref().profiles.clone();
                    if !values.eq(&profiles) {
                        *selected_users
                            .write() = SelectedProfiles {
                            profiles: event
                                .values()
                                .keys()
                                .into_iter()
                                .map(|v| v.clone())
                                .collect::<Vec<String>>(),
                        };
                    }
                },
                {users.read().deref().iter().map(|u| {
                    let checked = if let Some(_) = selected_users.read().profiles.clone().into_iter().find(|selected_p| selected_p.eq(&u.id)) { true } else { false } ;
                    rsx!(
                        label {
                            key: "{u.id}",
                            class: "group__checked-users",
                            input {
                                r#type: "checkbox",
                                id: "{u.id}",
                                name: "{u.id}",
                                checked: checked
                            }
                            RoomView {
                                displayname: "{u.displayname.clone()}",
                                avatar_uri: u.avatar_uri.clone(),
                                description: "",
                                on_click: move |_| {}
                            }
                        }
                    )
                })}
            }

            div { class: "group__cta__wrapper",
                Button {
                    text: translate!(i18, "group.select.cta"),
                    disabled: selected_users.read().profiles.is_empty(),
                    status: None,
                    on_click: move |_| { handle_complete_group.set(true) }
                }
            }
        }
    }
}
