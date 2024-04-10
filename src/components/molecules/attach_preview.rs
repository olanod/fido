use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use std::ops::Deref;

use crate::{
    components::atoms::{button::Variant, header_main::HeaderCallOptions, Button, Card, File},
    hooks::use_attach::use_attach,
    services::matrix::matrix::FileContent,
};

#[derive(PartialEq, Props, Clone)]
pub struct AttachPreviewProps {
    on_event: EventHandler<HeaderCallOptions>,
}

pub fn AttachPreview(props: AttachPreviewProps) -> Element {
    let i18 = use_i18();
    let attach = use_attach();

    let on_handle_card = move |_| props.on_event.call(HeaderCallOptions::CLOSE);

    match attach.get() {
        Some(file) => match file.content_type.type_() {
            mime::IMAGE => {
                rsx!(
                    article {
                        class: "attach__wrapper--image",
                        img {
                            class: "attach__content--image",
                            src: "{file.preview_url.deref()}"
                        }
                    }

                    Card {
                        file: "{file.preview_url.deref()}",
                        on_click: on_handle_card
                    }
                )
            }
            mime::VIDEO => {
                rsx!(
                    article {
                        class: "attach__wrapper--video",
                        video {
                            class: "attach__content--video",
                            src: "{file.preview_url.deref()}",
                            controls: true,
                            autoplay: true
                        }
                        div {
                            class: "attach__cta--video",
                            Button {
                                text: translate!(i18, "chat.attach_preview.cta.cancel"),
                                variant: Variant::Secondary,
                                status: None,
                                on_click: on_handle_card
                            }
                        }
                    }
                )
            }
            _ => {
                rsx!(
                    article {
                        class: "attach__wrapper--file",
                        div {
                            class: "attach__content--file",
                            h2 {
                                class: "attach__title--file",
                                {translate!(i18, "chat.attach_preview.title")}
                            }
                            div {
                                class: "attach__spacer",
                                File {
                                    body: FileContent {
                                        size: Some(file.size),
                                        body: file.name,
                                        source: None,
                                    },
                                    is_reply: false
                                }

                                div {
                                    class: "attach__spacer",
                                    Button {
                                        text: translate!(i18, "chat.attach_preview.cta.cancel"),
                                        variant: Variant::Secondary,
                                        status: None,
                                        on_click: on_handle_card
                                    }
                                }
                            }
                        }
                    }
                )
            }
        },
        None => {
            rsx!(
                div {
                    {translate!(i18, "chat.attach_preview.not_found")}
                }
            )
        }
    }
}
