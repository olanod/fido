use std::{collections::HashMap, ops::Deref};

use dioxus::{html::input_data::keyboard_types, prelude::*};
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};
use log::info;
use matrix_sdk::{
    config::SyncSettings,
    ruma::{OwnedUserId, UserId},
};

use crate::{
    components::atoms::{
        attach::AttachType, button::Variant, Attach, Avatar, Button, Close, Header, Icon,
        MessageInput, RoomView,
    },
    hooks::{
        use_attach::{use_attach, AttachFile},
        use_client::use_client,
    },
    pages::route::Route,
    services::matrix::matrix::create_room,
    utils::i18n_get_key_value::i18n_get_key_value,
    utils::matrix::{mxc_to_https_uri, ImageSize},
};
use futures_util::StreamExt;

#[derive(Clone, Debug)]
pub struct Profile {
    displayname: String,
    avatar_uri: Option<String>,
    id: String,
}

pub struct SelectedProfiles {
    profiles: Vec<String>,
}

pub fn RoomGroup(cx: Scope) -> Element {
    use_shared_state_provider::<SelectedProfiles>(cx, || SelectedProfiles { profiles: vec![] });
    use_shared_state_provider::<Option<AttachFile>>(cx, || None);

    let i18 = use_i18(cx);

    let key_common_error_thread_id = translate!(i18, "chat.common.error.thread_id");
    let key_common_error_event_id = translate!(i18, "chat.common.error.event_id");
    let key_common_error_room_id = translate!(i18, "chat.common.error.room_id");
    let key_common_error_user_id = translate!(i18, "chat.common.error.user_id");

    let key_group_error_not_found = translate!(i18, "group.error.not_found");
    let key_group_error_dm = translate!(i18, "group.error.dm");
    let key_group_error_profile = translate!(i18, "group.error.profile");
    let key_group_error_file = translate!(i18, "group.error.file");

    let key_group_title = "group-title";
    let key_group_select_label = "group-select-label";
    let key_group_select_placeholder = "group-select-placeholder";
    let key_group_select_cta = "group-select-cta";

    let key_group_meta_label = "group-meta-label";
    let key_group_meta_placeholder = "group-meta-placeholder";
    let key_group_meta_members_title = "group-meta-members-title";
    let key_group_meta_cta_back = "group-meta-cta-back";
    let key_group_meta_cta_create = "group-meta-cta-create";

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
    let user_id = use_state::<String>(cx, || String::from("@brayan-test-1:matrix.org"));
    let users = use_ref::<Vec<Profile>>(cx, || vec![]);
    let selected_users = use_shared_state::<SelectedProfiles>(cx).unwrap();
    let error = use_state::<Option<String>>(cx, || None);
    let error_creation = use_state::<Option<String>>(cx, || None);

    let handle_complete_group = use_ref::<bool>(cx, || false);
    let group_name = use_state::<String>(cx, || String::from(""));

    let task_search_user = use_coroutine(cx, |mut rx: UnboundedReceiver<String>| {
        to_owned![client, users];

        async move {
            while let Some(id) = rx.next().await {
                let element = users.read().clone().into_iter().find(|u| u.id.eq(&id));

                if let None = element {
<<<<<<< HEAD
                    let u = UserId::parse(&id).unwrap();
=======
                    let u = match UserId::parse(&id) {
                        Ok(id) => id,
                        Err(_) => {
                            notification.handle_error("{key_common_error_user_id}");
                            return;
                        }
                    };

>>>>>>> 190ae6f (ref(i18n): complete translations)
                    let u = u.deref();

                    let request =
                        matrix_sdk::ruma::api::client::profile::get_profile::v3::Request::new(u);
                    let resp = client.get().send(request, None).await;

                    match resp {
                        Ok(u) => users.with_mut(|x| {
                            let avatar_uri: Option<String> = if let Some(uri) = u.avatar_url {
                                mxc_to_https_uri(
                                    &uri,
                                    ImageSize {
                                        width: 48,
                                        height: 48,
                                    },
                                )
                            } else {
                                None
                            };

                            x.push(Profile {
<<<<<<< HEAD
                                displayname: String::from(u.displayname.unwrap()),
=======
                                displayname: match u.displayname {
                                    Some(d) => d,
                                    None => {
                                        notification.handle_error("{key_group_error_not_found}");
                                        return;
                                    }
                                },
>>>>>>> 190ae6f (ref(i18n): complete translations)
                                avatar_uri: avatar_uri,
                                id,
                            })
                        }),
                        Err(err) => {
                            info!("{err:?}");
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
                error_creation,
                navigation,
                key_group_error_dm
            ];

            async move {
                let users = selected_users
                    .read()
                    .profiles
                    .clone()
                    .into_iter()
                    .map(|p| UserId::parse(p).unwrap())
                    .collect::<Vec<OwnedUserId>>();

                let avatar = if let Some(file) = attach.get() {
                    Some(file.data)
                } else {
                    None
                };

                let name = group_name.get().clone();

                let room_meta =
                    create_room(&client.get(), false, &users, Some(name.clone()), avatar).await;

                info!("{room_meta:?}");

                match room_meta {
                    Ok(_) => {
                        let _ = client.get().sync_once(SyncSettings::default()).await;
                        navigation.push(Route::ChatList {});
                    }
                    Err(_) => {
                        let e = Some(key_group_error_dm);
                        error_creation.set(e)
                    }
                }
            }
        })
    };

    let on_handle_attach = move |event: Event<FormData>| {
        cx.spawn({
            to_owned![attach];

            async move {
                let files = &event.files;

                if let Some(f) = &files {
                    let fs = f.files();
<<<<<<< HEAD
                    let file = f.read_file(fs.get(0).unwrap()).await;
=======
                    let file_to_read = match fs.get(0) {
                        Some(file) => file,
                        None => {
                            notification.handle_error("{key_group_error_profile}");
                            return;
                        }
                    };
                    let file = f.read_file(file_to_read).await;
>>>>>>> 190ae6f (ref(i18n): complete translations)

                    if let Some(content) = file {
                        let blob = gloo::file::Blob::new(content.deref());
                        let object_url = gloo::file::ObjectUrl::from(blob);
<<<<<<< HEAD
                        // attach.set(Some(AttachFile {
                        //     name: fs.get(0).unwrap().to_string(),
                        //     preview_url: object_url,
                        //     data: content.clone(),
                        // }));
=======

                        let infer_type = infer::get(content.deref());

                        match infer_type {
                            Some(infered_type) => {
                                let content_type: Result<mime::Mime, _> =
                                    infered_type.mime_type().parse();
                                match content_type {
                                    Ok(content_type) => {
                                        let blob = match content_type.type_() {
                                            mime::IMAGE => gloo::file::Blob::new(content.deref()),
                                            _ => {
                                                notification.handle_error("{key_group_error_file}");
                                                return;
                                            }
                                        };

                                        let size = blob.size().clone();
                                        let object_url = gloo::file::ObjectUrl::from(blob);

                                        attach.set(Some(AttachFile {
                                            name: file_to_read.to_string(),
                                            preview_url: object_url,
                                            data: content.clone(),
                                            content_type,
                                            size,
                                        }));
                                    }
                                    _ => {
                                        notification.handle_error("{key_group_error_file}");
                                    }
                                }
                            }
                            None => {
                                notification.handle_error("{key_group_error_file}");
                            }
                        }
>>>>>>> 190ae6f (ref(i18n): complete translations)
                    }
                }
            }
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
            let element = if let Some(_) = attach.get()  {
                render!(rsx!(
                    img {
                        class: "group__attach",
                        src: "{attach.get_file().deref()}"
                    }
                ))
            } else {
                render!(rsx! (
                    Avatar{
                        name: if group_name.get().len() > 0 {String::from(group_name.get()) } else {String::from("X")},
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
                    if let Some(position) =selected_users.read().profiles.clone().into_iter().position(|selected_p| selected_p.eq(&u.id)) {
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
                    } else {
                        rsx!(span{})
                    }
                    })
                div {
                    class: "group__cta__wrapper row",
                    Button {
                        text: "{i18n_get_key_value(&i18n_map, key_group_meta_cta_back)}",
                        variant: &Variant::Secondary,
                        on_click: move |_| {
                            handle_complete_group.set(false)
                        }
                    }
                    Button {
                        text: "{i18n_get_key_value(&i18n_map, key_group_meta_cta_create)}",
                        disabled: group_name.get().len() == 0,
                        on_click: on_handle_create
                    }
                }
            )
        }else{
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
                        if event.code() == keyboard_types::Code::Enter && user_id.get().len() > 0 {
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
                        on_click: move |_| {
                            handle_complete_group.set(true)
                        }
                    }
                }
            )
        }
    }
}
