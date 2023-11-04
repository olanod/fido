use dioxus::prelude::*;
use log::info;
use std::ops::Deref;

use crate::{
    components::atoms::{Button, Card, File, button::Variant},
    hooks::use_attach::use_attach,
    services::matrix::matrix::FileContent,
};

pub fn AttachPreview<'a>(cx: Scope<'a>) -> Element<'a> {
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
        height: 100vh;
    "#;

    let on_handle_card = move |_| {
        attach.reset();
    };

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
                            height: 100vh;
                            display: flex;
                            justify-content: center;
                            align-items: center;
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
                                margin-top: 24px;
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
                            height: calc(100vh - 64px - 22px);
                            background: var(--background);
                            display: flex;
                            justify-content: center;
                            align-items: center;
                            padding: 24px;
                            flex-direction: column;
                            position: fixed;
                            top: 0;
                            left: 0;
                            width: 100vw;
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
