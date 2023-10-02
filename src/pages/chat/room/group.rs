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
                    let u = UserId::parse(&id).unwrap();
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
                                displayname: String::from(u.displayname.unwrap()),
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
                navigation
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
                        let e = Some(String::from("Ha ocurrido un error al crear el DM"));
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
                    let file = f.read_file(fs.get(0).unwrap()).await;

                    if let Some(content) = file {
                        let blob = gloo::file::Blob::new(content.deref());
                        let object_url = gloo::file::ObjectUrl::from(blob);
                        attach.set(Some(AttachFile {
                            name: fs.get(0).unwrap().to_string(),
                            preview_url: object_url,
                            data: content.clone(),
                        }));
                    }
                }
            }
        });
    };

    let button_style = r#"
        cursor: pointer;
        border: none;
        border-radius: 100%;
        max-width: 2.625rem;
        width: fit-content;
        height: 2.625rem;
        padding: 0;
    "#;

    render! {
        Header {
            text: "{i18n_get_key_value(&i18n_map, key_group_title)}",
            on_event: move |_|{
                navigation.go_back()
            }
        }
        if *handle_complete_group.read() {
            let attach_file_style = r#"
                height: 100%;
                width: 100%;
                object-fit: cover;
                position: relative;
                background: #000;
                border-radius: 100%;
            "#;

            let element = if let Some(_) = attach.get()  {
                render!(rsx!(
                    img {
                        style: "{attach_file_style}",
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
                    style: r#"
                        margin-top: 24px;
                    "#,
                    "{i18n_get_key_value(&i18n_map, key_group_meta_members_title)}"
                }
                users.read().deref().into_iter().map(|u| {
                    if let Some(position) =selected_users.read().profiles.clone().into_iter().position(|selected_p| selected_p.eq(&u.id)) {
                        rsx!(
                            div {
                                style: r#"
                                    display: flex;
                                    gap: 10px;
                                    justify-content: space-between;
                                    align-items: center;
                                "#,
                                RoomView {
                                    displayname: "{u.displayname.clone()}",
                                    avatar_uri: None,
                                    description: "",
                                    on_click: move |_| {

                                    }
                                }
                                button {
                                    style: "{button_style}",
                                    onclick: move |_| {
                                        selected_users.write().profiles.remove(position);
                                    },
                                    Icon {
                                        stroke: "#818898",
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
                    style: r#"
                        position: fixed;
                        background: white;
                        height: fit-content;
                        width: 100%;
                        padding: 12px 10px;
                        left: 0;
                        bottom: 0;
                    "#,
                    class: "row",
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
                    style: r#"
                        margin-top: 10px;
                    "#,
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
                                style: r#"
                                    display: flex;
                                    gap: 10px;
                                "#,
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
                    style: r#"
                        position: fixed;
                        background: white;
                        height: fit-content;
                        width: 100%;
                        padding: 12px 10px;
                        left: 0;
                        bottom: 0;

                    "#,
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
