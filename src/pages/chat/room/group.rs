use std::ops::Deref;

use dioxus::prelude::*;
use dioxus_router::{navigation, prelude::use_navigator};
use log::info;
use matrix_sdk::ruma::{OwnedUserId, UserId};

use crate::{
    components::atoms::{
        attach::AttachType, button::Variant, Attach, Avatar, Button, Close, Header, Icon,
        MessageInput, RoomView,
    },
    hooks::{
        use_attach::{use_attach, AttachFile},
        use_client::use_client,
    }, services::matrix::matrix::create_room,
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

    let navigation = use_navigator(cx);
    let client = use_client(cx);
    let attach = use_attach(cx);
    let user_id = use_state::<String>(cx, || String::from("@brayan-test-1:matrix.org"));
    let users = use_ref::<Vec<Profile>>(cx, || vec![]);
    let selected_users = use_shared_state::<SelectedProfiles>(cx).unwrap();
    let error = use_state::<Option<String>>(cx, || None);

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
                            x.push(Profile {
                                displayname: String::from(u.displayname.unwrap()),
                                avatar_uri: None,
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
            to_owned![client, selected_users, attach, group_name];

            async move {
                info!(
                    "selected: {:?}/n attach: {:?}/n name: {:?}",
                    selected_users.read().profiles,
                    group_name.get(),
                    attach.get().unwrap().data,
                );

                let users = selected_users
                    .read()
                    .profiles
                    .clone()
                    .into_iter()
                    .map(|p| UserId::parse(p).unwrap())
                    .collect::<Vec<OwnedUserId>>();

                let x = create_room(&client.get(), true, &users, None).await;

                info!("{x:?}")
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
            text: "New Group",
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
                        name: if group_name.get().len() > 0 {group_name.get()   } else {"X"},
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
                    itype: "text",
                    message: "{group_name.get()}",
                    placeholder: "eg: Uniqueorns",
                    label: "Name the group",
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
                    "Participants"
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
                        text: "Volver",
                        variant: &Variant::Secondary,
                        on_click: move |_| {
                            handle_complete_group.set(false)
                        }
                    }
                    Button {
                        text: "Crear grupo",
                        on_click: on_handle_create
                    }
                }
            )
        }else{
            rsx!(
                MessageInput{
                    itype: "text",
                    message: "{user_id.get()}",
                    placeholder: "Escribe el id",
                    error: error.get().as_ref(),
                    on_input: move |event: Event<FormData>| {
                        user_id.set(event.value.clone());
                    },
                    on_keypress: move |_| {
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
                        let checked = if let Some(_) =selected_users.read().profiles.clone().into_iter().find(|selected_p| selected_p.eq(&u.id)) {true} else {false} ;
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
                                    avatar_uri: None,
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
                        text: "Continuar",
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
