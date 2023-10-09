use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};
use log::info;
use std::{collections::HashMap, ops::Deref};

use crate::{
    components::atoms::{attach::AttachType, Attach, Avatar, Button, MessageInput, Spinner},
    hooks::{
        use_attach::{use_attach, AttachFile},
        use_client::use_client,
    },
    pages::route::Route,
    utils::i18n_get_key_value::i18n_get_key_value,
};

#[derive(Clone)]
pub struct Profile {
    displayname: String,
    avatar: Option<String>,
}

#[derive(Clone)]
pub struct SessionStatus {
    device_id: String,
    is_verified: bool,
}

#[derive(Clone)]
pub struct AdvancedInfo {
    homeserver: String,
    user_id: String,
    session: SessionStatus,
}

pub fn Profile(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    let key_username_label = "username-label";
    let key_username_placeholder = "username-placeholder";
    let key_username_cta_update = "username-cta_update";
    let key_management_title = "management-title";
    let key_management_deactivate_label = "management-deactivate-label";
    let key_management_deactivate_cta_deactivate = "management-deactivate-cta_deactivate";

    let i18n_map = HashMap::from([
        (
            key_username_label,
            translate!(i18, "profile.username.label"),
        ),
        (
            key_username_placeholder,
            translate!(i18, "profile.username.placeholder"),
        ),
        (
            key_username_cta_update,
            translate!(i18, "profile.username.cta_update"),
        ),
        (
            key_management_title,
            translate!(i18, "profile.management.title"),
        ),
        (
            key_management_deactivate_label,
            translate!(i18, "profile.management.deactivate.label"),
        ),
        (
            key_management_deactivate_cta_deactivate,
            translate!(i18, "profile.management.deactivate.cta_deactivate"),
        ),
    ]);

    use_shared_state_provider::<Option<AttachFile>>(cx, || None);

    let client = use_client(cx);
    let attach = use_attach(cx);
    let navigator = use_navigator(cx);

    let original_profile = use_ref::<Profile>(cx, || Profile {
        displayname: String::from(""),
        avatar: None,
    });
    let current_profile = use_ref::<Profile>(cx, || Profile {
        displayname: String::from(""),
        avatar: None,
    });
    let is_loading_profile = use_ref::<bool>(cx, || true);
    let advanced_info = use_ref::<AdvancedInfo>(cx, || AdvancedInfo {
        homeserver: String::from(""),
        user_id: String::from(""),
        session: SessionStatus {
            device_id: String::from(""),
            is_verified: true,
        },
    });

    use_coroutine(cx, |mut _rx: UnboundedReceiver<String>| {
        to_owned![
            client,
            original_profile,
            current_profile,
            is_loading_profile,
            advanced_info
        ];

        async move {
            let account_profile = client.get().account().get_profile().await.unwrap();

            let avatar_uri: Option<String> = match account_profile.avatar_url {
                Some(avatar) => {
                    let (server, id) = avatar.parts().unwrap();
                    let uri = format!("https://matrix-client.matrix.org/_matrix/media/r0/thumbnail/{}/{}?width=48&height=48&method=crop", server, id);
                    Some(String::from(uri))
                }
                None => None,
            };

            original_profile.set(Profile {
                displayname: account_profile.displayname.unwrap_or(String::from("")),
                avatar: avatar_uri,
            });

            current_profile.set(original_profile.read().deref().clone());

            let client = client.get();

            let homeserver = client.homeserver().await;
            let user_id = client.user_id();
            let device_id = client.session().unwrap().device_id;
            let mut is_session_verified = false;

            if let Ok(result) = client
                .encryption()
                .get_device(user_id.unwrap(), &device_id)
                .await
            {
                if let Some(device) = result {
                    is_session_verified = device.is_verified();
                }
            }

            advanced_info.set(AdvancedInfo {
                homeserver: String::from(homeserver),
                user_id: String::from(user_id.unwrap().to_string()),
                session: SessionStatus {
                    device_id: device_id.to_string(),
                    is_verified: is_session_verified,
                },
            });

            is_loading_profile.set(false);
        }
    });

    let attach_file_style = r#"
        height: 100%;
        width: 100%;
        object-fit: cover;
        border: 0.5px solid #0001;
        position: relative;
        background: var(--background-loud);
        border-radius: 100%;
    "#;

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

    let displayname = current_profile.read().deref().displayname.clone();
    let avatar = current_profile.read().avatar.clone();

    render! {
        if *is_loading_profile.read() {
            rsx!(
                div {
                    class: "spinner-dual-ring--center",
                    Spinner {}
                }
            )
        } else {
            let element = if let Some(_) = attach.get()  {
                render!(rsx!(
                    img {
                        style: "{attach_file_style}",
                        src: "{attach.get_file().deref()}"
                    }
                ))
            } else {

                render!(
                    rsx!(
                        Avatar {
                          name: displayname,
                          size: 80,
                          uri: avatar
                        }
                    )
                )
            };

            let message = current_profile.read().deref().displayname.clone();

            rsx!(
                section {
                    Attach {
                        atype: AttachType::Avatar(element),
                        on_click: on_handle_attach
                    }

                    div {
                        style: "
                            margin-top: 12px
                        ",
                        MessageInput{
                            message: "{message}",
                            placeholder: "{i18n_get_key_value(&i18n_map, key_username_placeholder)}",
                            label: "{i18n_get_key_value(&i18n_map, key_username_label)}",
                            error: None,
                            on_input: move |event: Event<FormData>| {
                                current_profile.with_mut(|p| p.displayname = event.value.clone() );
                            },
                            on_keypress: move |_| {
                            },
                            on_click: move |_| {

                            },
                        }
                    }
                    div {
                        style: "
                            margin-top: 24px
                        ",
                        Button {
                            text: "{i18n_get_key_value(&i18n_map, key_username_cta_update)}",
                            on_click: move |_| {
                                cx.spawn({
                                    to_owned![client, original_profile, current_profile, attach];

                                    async move {
                                        if !original_profile.read().displayname.eq(&current_profile.read().displayname) {
                                            let x = client.get().account().set_display_name(Some(current_profile.read().displayname.as_str())).await;
                                            info!("{x:?}");
                                            match x {
                                                Ok(_)=> {}
                                                Err(_)=> {}
                                            }
                                        }

                                        if let Some(y) = attach.get() {
                                            let x = client.get().account().upload_avatar(&mime::IMAGE_PNG, &y.data).await;
                                            info!("{x:?}");
                                            match x {
                                                Ok(url)=> {
                                                    let x = client.get().account().set_avatar_url(Some(&url)).await;
                                                    info!("{x:?}");
                                                }
                                                Err(_)=> {}
                                            }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }

                section {
                    style: r#"
                        margin-top: 40px;
                    "#,
                    h2 {
                        color: "var(--text-1)",
                        "Informacion de la cuenta"
                    }

                    h4 {
                        style: r#"
                            margin-top: 24px;
                            color: var(--text-1);
                        "#,
                        "Servidor"
                    }

                    p {
                        style: r#"
                            margin-top: 12px;
                            color: var(--text-2);
                        "#,
                        "{advanced_info.read().homeserver.deref()}"
                    }

                    h4 {
                        style: r#"
                            margin-top: 24px;
                            color: var(--text-1);
                        "#,
                        "ID del usuario de Matrix"
                    }

                    p {
                        style: r#"
                            margin-top: 12px;
                            color: var(--text-2);
                        "#,
                        "{advanced_info.read().user_id}"
                    }

                    h4 {
                        style: r#"
                            margin-top: 24px;
                            color: var(--text-1);
                        "#,
                        "Sesion ID"
                    }

                    p {
                        style: r#"
                            margin-top: 12px;
                            color: var(--text-2);
                        "#,
                        "{advanced_info.read().session.device_id}"
                    }
                    if !advanced_info.read().session.is_verified {
                        rsx!(
                            div {
                                style: "
                                    margin-top: 24px
                                ",
                                Button {
                                    text: "Verificar esta sesion",
                                    on_click: move |_| {
                                        navigator.push(Route::Verify { id: String::from("fidoid") });
                                    }
                                }
                            }
                        )
                    }
                }

                section {
                    style: r#"
                        margin-top: 40px;
                        color: var(--text-1);
                    "#,
                    h2 {
                        "{i18n_get_key_value(&i18n_map, key_management_title)}"
                    }

                    p {
                        style: r#"
                            margin-top: 12px;
                            color: var(--text-2);
                        "#,
                        "{i18n_get_key_value(&i18n_map, key_management_deactivate_label)}"
                    }
                    div {
                        style: "
                            margin-top: 24px
                        ",
                        Button {
                            text: "{i18n_get_key_value(&i18n_map, key_management_deactivate_cta_deactivate)}",
                            on_click: move |_| {
                                // cx.spawn({
                                //     to_owned!(client);

                                //     async move {
                                //         client.accoutn
                                //     }
                                // })
                            }
                        }
                    }
                }
            )
        }
    }
}
