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

    let attach_file_style = r#"
        height: 100%;
        width: 100%;
        object-fit: contain;
        border: 0.5px solid #0001;
        position: relative;
        background: var(--background-loud);
    "#;

    let attach_preview = r#"
        height: 100%;
    "#;

    let on_handle_card = move |_| cx.props.on_event.call(HeaderCallOptions::CLOSE);

    cx.render(rsx!(if let Some(file) = attach.get() {
        info!("{:?}", file.content_type.type_());

        match file.content_type.type_() {
            mime::IMAGE => {
                rsx!(
                    article {
                        style: "{attach_preview}",
                        img {
                            style: "{attach_file_style}",
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
                        style: "
                            height: 100%;
                            display: flex;
                            justify-content: center;
                            background: var(--background);
                            flex-direction: column;
                        ",
                        video {
                            style: "
                                height: 70%;
                            ",
                            src: "{attach.get_file().deref()}",
                            controls: true,
                            autoplay: true
                        }
                        div {
                            style: "
                                margin: 24px auto;
                                width: 50%;
                            ",
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
                        style: "
                            height: 100%;
                            background: var(--background);
                            display: flex;
                            justify-content: center;
                            align-items: center;
                            flex-direction: column;
                        ",
                        div {
                            style: "
                                background: var(--background-modal);
                                padding: 24px;
                                border-radius: 16px;
                            ",
                            h2 {
                                style: "
                                    color: var(--text-1);
                                    font-family: Inter;
                                    font-size: 24px;
                                    font-style: normal;
                                    font-weight: 500;
                                    line-height: 32px; /* 133.333% */
                                    letter-spacing: -0.24px;
                                    text-align: left;
                                ",
                                "Subir archivo"
                            }
                            div {
                                style: "margin-top: 24px;",
                                File {
                                    body: FileContent {
                                        size: Some(file.size),
                                        body: file.name,
                                        source: None,
                                    }
                                }

                                div {
                                    style: "margin-top: 24px;",
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
