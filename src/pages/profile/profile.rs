use dioxus::prelude::*;
use log::info;
use std::ops::Deref;

use crate::{
    components::atoms::{attach::AttachType, Attach, Avatar, Button, MessageInput, Spinner},
    hooks::{
        use_attach::{use_attach, AttachFile},
        use_client::use_client,
    },
};

#[derive(Clone)]
pub struct Profile {
    displayname: String,
    avatar: Option<String>,
}

pub fn Profile(cx: Scope) -> Element {
    use_shared_state_provider::<Option<AttachFile>>(cx, || None);

    let client = use_client(cx);
    let attach = use_attach(cx);
    let original_profile = use_ref::<Profile>(cx, || Profile {
        displayname: String::from(""),
        avatar: None,
    });
    let current_profile = use_ref::<Profile>(cx, || Profile {
        displayname: String::from(""),
        avatar: None,
    });
    let is_loading_profile = use_ref::<bool>(cx, || true);

    use_coroutine(cx, |mut _rx: UnboundedReceiver<String>| {
        to_owned![
            client,
            original_profile,
            current_profile,
            is_loading_profile
        ];

        async move {
            let account_profile = client.get().account().get_profile().await.unwrap();
            info!("{account_profile:?}");
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
            is_loading_profile.set(false);
        }
    });

    let attach_file_style = r#"
        height: 100%;
        width: 100%;
        object-fit: cover;
        border: 0.5px solid #0001;
        position: relative;
        background: #000;
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
                let displayname = current_profile.read().deref().displayname.clone();
                let avatar = current_profile.read().avatar.clone();
                let x = avatar.as_deref();

                render!(
                    rsx!(
                        Avatar {
                          name: "{displayname}",
                          size: 80,
                          uri: None
                        }
                    )
                )
            };

            let message = current_profile.read().deref().displayname.clone();

            rsx!(
                Attach {
                    atype: AttachType::Avatar(element),
                    on_click: on_handle_attach
                }

                MessageInput{
                    itype: "text",
                    message: "{message}",
                    placeholder: "eg: Uniqueorns",
                    label: "Name the group",
                    error: None,
                    on_input: move |event: Event<FormData>| {
                        current_profile.with_mut(|p| p.displayname = event.value.clone() );
                    },
                    on_keypress: move |_| {
                    },
                    on_click: move |_| {

                    },
                }
                div {
                    style: "
                        margin-top: 24px
                    ",
                    Button {
                        text: "Modificar perfil",
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

                h2 {
                    style: r#"
                        margin-top: 40px;
                    "#,
                    "Account management"
                }

                p {
                    style: r#"
                        margin-top: 12px;
                    "#,
                    "Desactivar cuenta"
                }
                div {
                    style: "
                        margin-top: 24px
                    ",
                    Button {
                        text: "Desactivar",
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

            )
        }
    }
}
