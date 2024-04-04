use dioxus::prelude::*;

use crate::{
    services::matrix::matrix::{FileContent, ImageType},
    utils::vec_to_url::vec_to_url,
};

#[derive(PartialEq, Props, Clone)]
pub struct ImageProps {
    body: FileContent,
    is_reply: bool,
}

pub fn ImageMessage(props: ImageProps) -> Element {
    let message__content__image = if props.is_reply {
        "message__content__image--is-replying message-reply__content--media"
    } else {
        "message__content__image--not-replying"
    };

    match props.body.source {
        Some(ImageType::URL(url)) => rsx!( img { class: "{message__content__image}", src: "{url}" } ),
        Some(ImageType::Media(content)) => {
            let url = vec_to_url(content);

            match url {
                Ok(url) => rsx!(
                    img { class: "{message__content__image}", src: "{url}" }
                    a { href: "{url}" }
                ),
                Err(_) => rsx!( strong { "Unable to read file" } ),
            }
        }
        None => rsx!( strong { "File Not Found" } ),
    }
}
