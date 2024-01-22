use dioxus::prelude::*;
use log::info;
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
    let attach = use_attach(cx);

    let on_handle_card = move |_| cx.props.on_event.call(HeaderCallOptions::CLOSE);

    cx.render(rsx!(if let Some(file) = attach.get() {
        info!("{:?}", file.content_type.type_());

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
                        }
                        div {
                            class: "attach__cta--video",
                            Button {
                                text: "Cancelar",
                                variant: &Variant::Secondary,
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
                                "Subir archivo"
                            }
                            div {
                                class: "attach__spacer",
                                File {
                                    body: FileContent {
                                        size: Some(file.size),
                                        body: file.name,
                                        source: None,
                                    }
                                }

                                div {
                                    class: "attach__spacer",
                                    Button {
                                        text: "Cancelar",
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
