use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use std::ops::Deref;

use crate::{
    components::atoms::{button::Variant, header_main::HeaderCallOptions, Button, Card, File},
    hooks::use_attach::use_attach,
    services::matrix::matrix::FileContent,
};

#[derive(Props)]
pub struct AttachPreviewProps<'a> {
    on_event: EventHandler<'a, HeaderCallOptions>,
}

pub fn AttachPreview<'a>(cx: Scope<'a, AttachPreviewProps<'a>>) -> Element<'a> {
    let i18 = use_i18(cx);
    let attach = use_attach(cx);

    let key_attach_preview_cta_cancel = translate!(i18, "chat.attach_preview.cta.cancel");

    let on_handle_card = move |_| cx.props.on_event.call(HeaderCallOptions::CLOSE);

    cx.render(rsx!(if let Some(file) = attach.get() {
        match file.content_type.type_() {
            mime::IMAGE => {
                rsx!(
                    article {
                        class: "attach__wrapper--image",
                        img {
                            class: "attach__content--image",
                            src: "{attach.get_file().deref()}"
                        }
                    }

<<<<<<< HEAD
                    Card {
                        file: "{attach.get_file().deref()}",
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
                            src: "{attach.get_file().deref()}",
                            controls: true,
                            autoplay: true
=======
                        Card {
                            file: "{file.deref()}",
                            on_click: on_handle_card
                        }
                    )
                } else {
                    rsx!(
                        div {
                            translate!(i18, "chat.attach_preview.not_found")
                        }
                    )
                }
            }
            mime::VIDEO => {
                if let Ok(file) = attach.get_file() {
                    rsx!(
                        article {
                            class: "attach__wrapper--video",
                            video {
                                class: "attach__content--video",
                                src: "{file.deref()}",
                                controls: true,
                                autoplay: true
                            }
                            div {
                                class: "attach__cta--video",
                                Button {
                                    text: "{key_attach_preview_cta_cancel}",
                                    variant: &Variant::Secondary,
                                    on_click: on_handle_card
                                }
                            }
>>>>>>> 190ae6f (ref(i18n): complete translations)
                        }
                        div {
<<<<<<< HEAD
                            class: "attach__cta--video",
                            Button {
                                text: "Cancelar",
                                variant: &Variant::Secondary,
                                on_click: on_handle_card
                            }
=======
                            translate!(i18, "chat.attach_preview.not_found")
>>>>>>> 190ae6f (ref(i18n): complete translations)
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
                                translate!(i18, "chat.attach_preview.title")
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
                                        text: "{key_attach_preview_cta_cancel}",
                                        variant: &Variant::Secondary,
                                        on_click: on_handle_card
                                    }
                                }
                            }
                        }
                    }
                )
            }
        }
    }))
}
