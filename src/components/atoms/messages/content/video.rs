use std::ops::Deref;

use dioxus::prelude::*;
use gloo::file::BlobContents;
use web_sys::Url;

use crate::{
    components::atoms::File,
    services::matrix::matrix::{FileContent, ImageType},
};

#[derive(PartialEq, Props)]
pub struct VideoProps<'a> {
    body: &'a FileContent,
    is_reply: bool,
}

pub fn VideoMessage<'a>(cx: Scope<'a, VideoProps<'a>>) -> Element<'a> {
    let message__content__video = if cx.props.is_reply {
        "message__content__video--is-replying message-reply__content--video"
    } else {
        "message__content__video--not-replying"
    };

    render!(if cx.props.is_reply {
        match cx.props.body.source.as_ref().unwrap() {
            ImageType::URL(url) => {
                rsx!(video {
                    class: "{message__content__video}",
                    src: "{url}",
                    controls: true,
                    autoplay: false
                })
            }
            ImageType::Media(content) => {
                let c = content.deref();
                let parts = js_sys::Array::of1(&unsafe { c.into_jsvalue() });
                let blob = web_sys::Blob::new_with_u8_array_sequence(&parts).unwrap();
                let url = Url::create_object_url_with_blob(&blob).unwrap();

                rsx!(video {
                    class: "{message__content__video}",
                    src: "{url}",
                    controls: true,
                    autoplay: false
                })
            }
        }
    } else {
        rsx!(File {
            body: cx.props.body.clone(),
            is_reply: cx.props.is_reply,
        })
    })
}
