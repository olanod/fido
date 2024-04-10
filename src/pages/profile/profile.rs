use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};
use std::ops::Deref;

use crate::{
    components::atoms::{attach::AttachType, Attach, Avatar, Button, MessageInput, Spinner},
    hooks::{
        use_attach::{use_attach, AttachError, AttachFile},
        use_client::use_client,
        use_notification::use_notification,
    },
    pages::route::Route,
    utils::matrix::{mxc_to_thumbnail_uri, ImageMethod, ImageSize},
};

use futures_util::TryFutureExt;

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

#[derive(Clone, Debug)]
pub enum ProfileError {
    InvalidUserId,
    InvalidDeviceId,
    UserNotFound,
    InvalidUsername,
    ServerError,
}

pub fn Profile() -> Element {
    let i18 = use_i18();

    use_context_provider::<Signal<Option<AttachFile>>>(|| Signal::new(None));

    let client = use_client();
    let navigator = use_navigator();
    let mut attach = use_attach();
    let mut notification = use_notification();

    let mut original_profile = use_signal::<Profile>(|| Profile {
        displayname: String::from(""),
        avatar: None,
    });
    let mut current_profile = use_signal::<Profile>(|| Profile {
        displayname: String::from(""),
        avatar: None,
    });
    let mut is_loading_profile = use_signal::<bool>(|| true);
    let mut advanced_info = use_signal::<AdvancedInfo>(|| AdvancedInfo {
        homeserver: String::from(""),
        user_id: String::from(""),
        session: SessionStatus {
            device_id: String::from(""),
            is_verified: true,
        },
    });

    use_coroutine(|mut _rx: UnboundedReceiver<String>| {
        async move {
            let client = client.get();

            let account_profile = client
                .account()
                .get_profile()
                .await
                .map_err(|_| ProfileError::UserNotFound)?;

            let avatar_uri: Option<String> = account_profile.avatar_url.and_then(|uri| {
                mxc_to_thumbnail_uri(&uri, ImageSize::default(), ImageMethod::SCALE)
            });

            original_profile.set(Profile {
                displayname: account_profile.displayname.map_or(String::from(""), |d| d),
                avatar: avatar_uri,
            });

            current_profile.set(original_profile.read().deref().clone());

            let homeserver = client.homeserver().await;
            let user_id = client.user_id().ok_or(ProfileError::InvalidUserId)?;
            let device_id = client
                .session()
                .ok_or(ProfileError::InvalidDeviceId)?
                .device_id;

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

            Ok::<(), ProfileError>(())
        }
        .unwrap_or_else(move |e: ProfileError| {
            let message = match e {
                ProfileError::UserNotFound => translate!(i18, "profile.error.not_found"),
                ProfileError::InvalidUsername => translate!(i18, "profile.error.profile"),
                ProfileError::InvalidUserId => translate!(i18, "chat.common.error.user_id"),
                ProfileError::ServerError => translate!(i18, "chat.common.error.device_id"),
                ProfileError::InvalidDeviceId => translate!(i18, "chat.common.error.server"),
            };

            notification.handle_error(&message);
        })
    });

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
                    AttachError::UnknownContent => {
                        translate!(i18, "chat.input_message.unknown_content")
                    }
                };

                notification.handle_error(&message_error);
            })
        });
    };

    let displayname = current_profile.read().deref().displayname.clone();
    let avatar = current_profile.read().avatar.clone();

    rsx! {
        if *is_loading_profile.read() {

            div { class: "spinner-dual-ring--center", Spinner {} }
        } else {
            {let element = if let Ok(file) = attach.get_file()  {
                rsx!(
                    img {
                        class: "profile__attach",
                        src: "{file.deref()}"
                    }
                )
            } else {
                rsx!(
                    Avatar {
                        name: displayname,
                        size: 80,
                        uri: avatar
                    }
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
                            placeholder: translate!(i18, "profile.username.placeholder"),
                            label: translate!(i18, "profile.username.label"),
                            error: None,
                            on_input: move |event: Event<FormData>| {
                                current_profile.with_mut(|p| p.displayname = event.value().clone() );
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
                            text: translate!(i18, "profile.username.cta_update"),
                            status: None,
                            on_click: move |_| {
                                spawn({
                                    async move {
                                        if !original_profile.read().displayname.eq(&current_profile.read().displayname) {
                                            let x = client.get().account().set_display_name(Some(current_profile.read().displayname.as_str())).await;
                                            log::info!("{x:?}");
                                            match x {
                                                Ok(_)=> {}
                                                Err(_)=> {}
                                            }
                                        }
            
                                        if let Some(y) = attach.get() {
                                            let x = client.get().account().upload_avatar(&mime::IMAGE_PNG, &y.data).await;
                                            log::info!("{x:?}");
                                            match x {
                                                Ok(url)=> {
                                                    let x = client.get().account().set_avatar_url(Some(&url)).await;
                                                    log::info!("{x:?}");
                                                }
                                                Err(_)=> {}
                                            }
                                        }
                                    }
                                });
                            }
                        }
                    }
                }
            
                section {
                    class: "profile__section",
                    h2 {
                        class: "profile__title",
                        {translate!(i18, "profile.management.info.subtitle")}
                    }
            
                    h4 {
                        class: "profile__subtitle",
                        {translate!(i18, "profile.management.info.label_1")}
                    }
            
                    p {
                        class: "profile__content",
                        "{advanced_info.read().homeserver.deref()}"
                    }
            
                    h4 {
                        class: "profile__subtitle",
                        {translate!(i18, "profile.management.info.label_2")}
                    }
            
                    p {
                        class: "profile__content",
                        "{advanced_info.read().user_id}"
                    }
            
                    h4 {
                        class: "profile__subtitle",
                        {translate!(i18, "profile.management.info.label_3")}
                    }
            
                    p {
                        class: "profile__content",
                        "{advanced_info.read().session.device_id}"
                    }
                    if !advanced_info.read().session.is_verified {
            
                            div {
                                class: "profile__cta",
                                Button {
                                    text: translate!(i18, "profile.management.info.cta"),
                                    status: None,
                                    on_click: move |_| {
                                        navigator.push(Route::Verify { id: String::from("fidoid") });
                                    }
                                }
                            }
            
                    }
                }
            
                section {
                    class: "profile__section",
                    h2 {
                        {translate!(i18, "profile.management.title")}
                    }
            
                    p {
                        class: "profile__content",
                        {translate!(i18, "profile.management.deactivate.label")}
                    }
                    div {
                        class: "profile__cta",
                        Button {
                            text: translate!(i18, "profile.management.deactivate.cta_deactivate"),
                            status: None,
                            on_click: move |_| {
                                // spawn({
                                //     to_owned!(client);
            
                                //     async move {
                                //         client.accoutn
                                //     }
                                // })
                            }
                        }
                    }
                }
            )}
        }
    }
}
