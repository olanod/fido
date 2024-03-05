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
        molecules::rooms::CurrentRoom,
    },
    hooks::{
        use_attach::{use_attach, AttachError, AttachFile},
        use_client::{use_client, UseClientState},
        use_notification::use_notification,
        use_room::use_room,
    },
    pages::chat::room::new::CreationStatus,
    services::matrix::matrix::create_room,
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

pub fn RoomGroup(cx: Scope) -> Element {
    use_shared_state_provider::<SelectedProfiles>(cx, || SelectedProfiles { profiles: vec![] });
    use_shared_state_provider::<Option<AttachFile>>(cx, || None);

    let i18 = use_i18(cx);

    let key_common_error_user_id = translate!(i18, "chat.common.error.user_id");
    let key_common_error_server = translate!(i18, "chat.common.error.server");
    let key_group_error_not_found = translate!(i18, "group.error.not_found");
    let key_group_error_profile = translate!(i18, "group.error.profile");

    let key_group_title = "group-title";
    let key_group_select_label = "group-select-label";
    let key_group_select_placeholder = "group-select-placeholder";
    let key_group_select_cta = "group-select-cta";

    let key_group_meta_label = "group-meta-label";
    let key_group_meta_placeholder = "group-meta-placeholder";
    let key_group_meta_members_title = "group-meta-members-title";
    let key_group_meta_cta_back = "group-meta-cta-back";
    let key_group_meta_cta_create = "group-meta-cta-create";

    let key_input_message_unknown_content = translate!(i18, "chat.input_message.unknown_content");
    let key_input_message_file_type = translate!(i18, "chat.input_message.file_type");
    let key_input_message_not_found = translate!(i18, "chat.input_message.not_found");

    let i18n_map = HashMap::from([
        (key_group_title, translate!(i18, "group.title")),
        (
            key_group_select_label,
            translate!(i18, "group.select.label"),
        ),
        (
            key_group_select_placeholder,
            translate!(i18, "group.select.placeholder"),
        ),
        (key_group_select_cta, translate!(i18, "group.select.cta")),
        (key_group_meta_label, translate!(i18, "group.meta.label")),
        (
            key_group_meta_placeholder,
            translate!(i18, "group.meta.placeholder"),
        ),
        (
            key_group_meta_members_title,
            translate!(i18, "group.meta.members.title"),
        ),
        (
            key_group_meta_cta_back,
            translate!(i18, "group.meta.cta.back"),
        ),
        (
            key_group_meta_cta_create,
            translate!(i18, "group.meta.cta.create"),
        ),
    ]);

    let navigation = use_navigator(cx);
    let client = use_client(cx);
    let attach = use_attach(cx);
    let notification = use_notification(cx);
    let room = use_room(cx);

    let selected_users =
        use_shared_state::<SelectedProfiles>(cx).expect("Unable to use SelectedProfile");

    let user_id = use_state::<String>(cx, || String::from(""));
    let users = use_ref::<Vec<Profile>>(cx, || vec![]);

    let error = use_state::<Option<String>>(cx, || None);

    let handle_complete_group = use_ref::<bool>(cx, || false);
    let group_name = use_state::<String>(cx, || String::from(""));
    let status = use_state::<CreationStatus>(cx, || CreationStatus::Start);

    let task_search_user = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![client, users, notification, key_group_error_not_found];

        async move {
            while let Some(id) = rx.next().await {
                let element = users.read().clone().into_iter().find(|u| u.id.eq(&id));

                if let None = element {
                    match process_find_user_by_id(&id, &client).await {
                        Ok(profile) => users.with_mut(|user| user.push(profile)),
                        Err(_) => {
                            notification.handle_error(&key_group_error_not_found);
                        }
                    }
                }
            }
        }
    });

    let on_handle_create = move |_| {
        cx.spawn({
            to_owned![
                client,
                selected_users,
                attach,
                group_name,
                
                navigation,
                key_common_error_user_id,
                key_group_error_profile,
                key_group_error_not_found,
                key_common_error_server,
                
                notification,
                status,
                room
            ];

            let status_error = status.clone();

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
                let name = group_name.get().clone();

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
                    CreateRoomError::InvalidUserId => &key_common_error_user_id,
                    CreateRoomError::UserNotFound => &key_group_error_profile,
                    CreateRoomError::InvalidUsername => &key_group_error_not_found,
                    CreateRoomError::ServerError => &key_common_error_server,
                };

                status_error.set(CreationStatus::Error(e));
                notification.handle_error(&message_error);
            })
        })
    };

    let on_handle_attach = move |event: Event<FormData>| {
        cx.spawn({
            to_owned![
                attach,
                notification,
                key_input_message_not_found,
                key_input_message_file_type,
                key_input_message_unknown_content
            ];

            async move {
                let files = &event.files.clone().ok_or(AttachError::NotFound)?;
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
                    AttachError::NotFound => key_input_message_not_found,
                    AttachError::UncoverType => key_input_message_file_type,
                    AttachError::UnknownContent => key_input_message_unknown_content,
                };

                notification.handle_error(&message_error);
            })
        });
    };

    render! {
        Header {
            text: "{i18n_get_key_value(&i18n_map, key_group_title)}",
            on_event: move |_|{
                navigation.go_back()
            }
        }
        if *handle_complete_group.read() {
            let element = if let Ok(file) = attach.get_file()  {
                render!(rsx!(
                    img {
                        class: "group__attach",
                        src: "{file.deref()}"
                    }
                ))
            } else {
                render!(rsx! (
                    Avatar{
                        name: if !group_name.get().is_empty() {String::from(group_name.get()) } else {String::from("X")},
                        size: 80,
                        uri: None
                    }
                ))
            };

            rsx!(
                Attach{
                    atype: AttachType::Avatar(element),
                    on_click: on_handle_attach
                }

                MessageInput{
                    message: "{group_name.get()}",
                    placeholder: "{i18n_get_key_value(&i18n_map, key_group_meta_placeholder)}",
                    label: "{i18n_get_key_value(&i18n_map, key_group_meta_label)}",
                    error: error.get().as_ref(),
                    on_input: move |event: Event<FormData>| {
                        group_name.set(event.value.clone());
                    },
                    on_keypress: move |_| {
                    },
                    on_click: move |_| {

                    },
                }
                p {
                    class: "group__title",
                    "{i18n_get_key_value(&i18n_map, key_group_meta_members_title)}"
                }
                users.read().deref().into_iter().map(|u| {
                    selected_users.read().profiles.clone().into_iter().position(|selected_p| selected_p.eq(&u.id)).map(|position| {
                        render!(
                            rsx!(
                                div {
                                    class: "group__users",
                                    RoomView {
                                        displayname: "{u.displayname.clone()}",
                                        avatar_uri: None,
                                        description: "",
                                        on_click: move |_| {

                                        }
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
                        )
                    }).flatten()
                })
                if !matches!(status.get(), CreationStatus::Start) {
                    rsx!(
                        match status.get() {
                            CreationStatus::Creating => {
                                render!(rsx! {
                                    div {
                                        class: "room-new__status-container",
                                        p {
                                            class: "room-new__status__description",
                                            translate!(i18, "group.status.creating")
                                        }
                                    }
                                })
                            },
                            CreationStatus::Ok => {
                                render!(rsx! {
                                    div {
                                        class: "room-new__status-container",
                                        p {
                                            class: "room-new__status__description",
                                            translate!(i18, "group.status.created")
                                        }
                                    }
                                })
                            },
                            CreationStatus::Error(CreateRoomError::ServerError) => {
                                let cta_back = translate!(i18, "group.status.error.cta.back");
                                let cta_try = translate!(i18, "group.status.error.cta.try");
                                render!(rsx! {
                                    div {
                                        class: "room-new__status-container",
                                        h3 {
                                            class: "room-new__status__title",
                                            translate!(i18, "group.status.error.title")
                                        }
                                        p {
                                            class: "room-new__status__description",
                                            translate!(i18, "group.status.error.description")
                                        }
                                        div {
                                            class: "row room-new__status-cta",
                                            Button{
                                                text: "{cta_back}",
                                                variant: &Variant::Secondary,
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
                                })
                            },
                            _ => None
                        }
                    )
                } else {
                    rsx!(
                        div {
                            class: "group__cta__wrapper row",
                            Button {
                                text: "{i18n_get_key_value(&i18n_map, key_group_meta_cta_back)}",
                                status: None,
                                variant: &Variant::Secondary,
                                on_click: move |_| {
                                    handle_complete_group.set(false)
                                }
                            }
                            Button {
                                text: "{i18n_get_key_value(&i18n_map, key_group_meta_cta_create)}",
                                status: None,
                                disabled: group_name.get().len() == 0,
                                on_click: on_handle_create
                            }
                        }
                    )
                }
            )
        } else {
            rsx!(
                MessageInput{
                    message: "{user_id.get()}",
                    placeholder: "{i18n_get_key_value(&i18n_map, key_group_select_placeholder)}",
                    label: "{i18n_get_key_value(&i18n_map, key_group_select_label)}",
                    error: error.get().as_ref(),
                    on_input: move |event: Event<FormData>| {
                        user_id.set(event.value.clone());
                    },
                    on_keypress: move |event: KeyboardEvent| {
                        if event.code() == keyboard_types::Code::Enter && !user_id.get().is_empty() {
                            let id = user_id.get();
                            task_search_user.send(id.to_string())
                        }
                    },
                    on_click: move |_| {
                        let id = user_id.get();
                        task_search_user.send(id.to_string())
                    },
                }
                form {
                    class: "group__form",
                    onchange: move |event| {
                        let values = event.values.clone().into_keys().collect::<Vec<String>>();
                        let profiles = selected_users.read().deref().profiles.clone();

                        if !values.eq(&profiles) {
                            *selected_users.write() = SelectedProfiles {
                                profiles: event.values.keys().into_iter().map(|v| v.clone()).collect::<Vec<String>>()
                            };
                        }
                    },
                    users.read().deref().iter().map(|u| {
                        let checked = if let Some(_) = selected_users.read().profiles.clone().into_iter().find(|selected_p| selected_p.eq(&u.id)) {true} else {false} ;

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
                                    on_click: move |_| {

                                    }
                                }
                            }
                        )
                    })
                }
                div {
                    class: "group__cta__wrapper",
                    Button {
                        text: "{i18n_get_key_value(&i18n_map, key_group_select_cta)}",
                        disabled: if selected_users.read().profiles.len() == 0 { true } else { false },
                        status: None,
                        on_click: move |_| {
                            handle_complete_group.set(true)
                        }
                    }
                }
            )
        }
    }
}

pub(crate) async fn process_find_user_by_id(
    id: &str,
    client: &UseClientState,
) -> Result<Profile, CreateRoomError> {
    let u = UserId::parse(&id).map_err(|_| CreateRoomError::InvalidUserId)?;

    let u = u.deref();

    let request = matrix_sdk::ruma::api::client::profile::get_profile::v3::Request::new(u);

    let response = client
        .get()
        .send(request, None)
        .await
        .map_err(|_| CreateRoomError::UserNotFound)?;

    let displayname = response
        .displayname
        .ok_or(CreateRoomError::InvalidUsername)?;

    let avatar_uri = response
        .avatar_url
        .map(|uri| {
            mxc_to_thumbnail_uri(
                &uri,
                ImageSize {
                    width: 48,
                    height: 48,
                },
                ImageMethod::CROP,
            )
        })
        .flatten();

    let profile = Profile {
        displayname,
        avatar_uri,
        id: id.to_string(),
    };

    Ok(profile)
}
