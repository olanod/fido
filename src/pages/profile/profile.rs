use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};
use log::info;
use std::{collections::HashMap, ops::Deref};

use crate::{
    components::atoms::{
        attach::AttachType, notification, Attach, Avatar, Button, MessageInput, Spinner,
    },
    hooks::{
        use_attach::{use_attach, AttachFile},
        use_client::use_client,
        use_notification::{
            use_notification, NotificationHandle, NotificationItem, NotificationType,
        },
    },
    pages::route::Route,
    utils::{
        i18n_get_key_value::i18n_get_key_value,
        matrix::{mxc_to_thumbnail_uri, ImageMethod, ImageSize},
    },
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

    let key_common_error_thread_id = translate!(i18, "chat.common.error.thread_id");
    let key_common_error_event_id = translate!(i18, "chat.common.error.event_id");
    let key_common_error_room_id = translate!(i18, "chat.common.error.room_id");
    let key_common_error_user_id = translate!(i18, "chat.common.error.user_id");
    let key_common_error_device_id = translate!(i18, "chat.common.error.device_id");

    let key_management_info_cta = translate!(i18, "profile.management.info.cta");

    let key_profile_error_not_found = translate!(i18, "profile.error.not_found");
    let key_profile_error_profile = translate!(i18, "profile.error.profile");
    let key_profile_error_file = translate!(i18, "profile.error.file");

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
    let notification = use_notification(cx);

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
            advanced_info,
            notification
        ];

        async move {
            let profile = client.get().account().get_profile().await;

            let account_profile = match profile {
                Ok(p) => p,
                Err(_) => {
                    notification.handle_error("{key_profile_error_profile}");
                    return;
                }
            };

            let avatar_uri: Option<String> = account_profile.avatar_url.and_then(|uri| {
                mxc_to_thumbnail_uri(&uri, ImageSize::default(), ImageMethod::SCALE)
            });

            original_profile.set(Profile {
                displayname: match account_profile.displayname {
                    Some(d) => d,
                    None => String::from(""),
                },
                avatar: avatar_uri,
            });

            current_profile.set(original_profile.read().deref().clone());

            let client = client.get();

            let homeserver = client.homeserver().await;
            let user_id = match client.user_id() {
                Some(id) => id,
                None => {
                    notification.handle_error("{key_common_error_user_id}");
                    return;
                }
            };

            let device_id = match client.session() {
                Some(s) => s.device_id,
                None => {
                    notification.handle_error("{key_common_error_device_id}");
                    return;
                }
            };
            let mut is_session_verified = false;

            if let Ok(result) = client.encryption().get_device(user_id, &device_id).await {
                if let Some(device) = result {
                    is_session_verified = device.is_verified();
                }
            }

            advanced_info.set(AdvancedInfo {
                homeserver: String::from(homeserver),
                user_id: String::from(user_id.to_string()),
                session: SessionStatus {
                    device_id: device_id.to_string(),
                    is_verified: is_session_verified,
                },
            });

            is_loading_profile.set(false);
        }
    });

    let on_handle_attach = move |event: Event<FormData>| {
        cx.spawn({
            to_owned![attach, notification];

            async move {
                let files = &event.files;

                if let Some(f) = &files {
                    let fs = f.files();
                    let file_to_read = match fs.get(0) {
                        Some(file) => file,
                        None => {
                            notification.handle_error("key_profile_error_profile");
                            return;
                        }
                    };
                    let file = f.read_file(file_to_read).await;

                    if let Some(content) = file {
                        let blob = gloo::file::Blob::new(content.deref());
                        let object_url = gloo::file::ObjectUrl::from(blob);

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
                                                notification
                                                    .handle_error("{key_profile_error_file}");
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
                                        notification.handle_error("{key_profile_error_file}");
                                    }
                                }
                            }
                            None => {
                                notification.handle_error("{key_profile_error_file}");
                            }
                        }
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
            let element = if let Ok(file) = attach.get_file()  {
                render!(rsx!(
                    img {
                        class: "profile__attach",
                        src: "{file.deref()}"
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
                        class: "profile__input",
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
                        class: "profile__cta",
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
                    class: "profile__section",
                    h2 {
                        class: "profile__title",
                        translate!(i18, "profile.management.info.subtitle")
                    }

                    h4 {
                        class: "profile__subtitle",
                        translate!(i18, "profile.management.info.label_1")
                    }

                    p {
                        class: "profile__content",
                        "{advanced_info.read().homeserver.deref()}"
                    }

                    h4 {
                        class: "profile__subtitle",
                        translate!(i18, "profile.management.info.label_2")
                    }

                    p {
                        class: "profile__content",
                        "{advanced_info.read().user_id}"
                    }

                    h4 {
                        class: "profile__subtitle",
                        translate!(i18, "profile.management.info.label_3")
                    }

                    p {
                        class: "profile__content",
                        "{advanced_info.read().session.device_id}"
                    }
                    if !advanced_info.read().session.is_verified {
                        rsx!(
                            div {
                                class: "profile__cta",
                                Button {
                                    text: "{key_management_info_cta}",
                                    on_click: move |_| {
                                        navigator.push(Route::Verify { id: String::from("fidoid") });
                                    }
                                }
                            }
                        )
                    }
                }

                section {
                    class: "profile__section",
                    h2 {
                        "{i18n_get_key_value(&i18n_map, key_management_title)}"
                    }

                    p {
                        class: "profile__content",
                        "{i18n_get_key_value(&i18n_map, key_management_deactivate_label)}"
                    }
                    div {
                        class: "profile__cta",
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
