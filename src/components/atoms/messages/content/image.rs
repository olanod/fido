use std::ops::Deref;

use dioxus::prelude::*;
use gloo::file::BlobContents;
use web_sys::Url;

use crate::services::matrix::matrix::{FileContent, ImageType};

#[derive(PartialEq, Props)]
pub struct ImageProps<'a> {
    body: &'a FileContent,
    is_reply: bool,
}

pub fn ImageMessage<'a>(cx: Scope<'a, ImageProps<'a>>) -> Element<'a> {
    let message__content__image = if cx.props.is_reply {
        "message__content__image--is-replying message-reply__content--media"
    } else {
        "message__content__image--not-replying"
    };

    render!(match cx.props.body.source.clone().unwrap() {
        ImageType::URL(url) => {
            rsx!(img {
                class: "{message__content__image}",
                src: "{url}"
            })
        }
        ImageType::Media(content) => {
            let c = content.deref();
            let parts = js_sys::Array::of1(&unsafe { c.into_jsvalue() });
            let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).unwrap();
            let url = Url::create_object_url_with_blob(&blob).unwrap();

            rsx!(
              img {
                class: "{message__content__image}",
                src: "{url}"
              }
              a {
                href: "{url}",
              }
            )
        }
    })
}
