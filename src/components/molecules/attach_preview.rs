use dioxus::prelude::*;
use std::ops::Deref;

use crate::{components::atoms::Card, hooks::use_attach::use_attach};

pub fn AttachPreview<'a>(cx: Scope<'a>) -> Element<'a> {
    let attach = use_attach(cx);

    let attach_file_style = r#"
        height: 100%;
        width: 100%;
        object-fit: contain;
        border: 0.5px solid #0001;
        position: relative;
        background: #000;
    "#;

    let attach_preview = r#"
        height: 100vh;
    "#;

    let on_handle_card = move |_| {
        attach.reset();
    };

    cx.render(rsx!(
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
    ))
}
