use crate::{
    components::atoms::File,
    services::matrix::matrix::{FileContent, ImageType},
    utils::vec_to_url::vec_to_url,
};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct VideoProps {
    body: FileContent,
    is_reply: bool,
}

pub fn VideoMessage(props: VideoProps) -> Element {
    let message__content__video = if props.is_reply {
        "message__content__video--is-replying message-reply__content--video"
    } else {
        "message__content__video--not-replying"
    };

    if !props.is_reply {
        match props.body.source {
            Some(ImageType::URL(url)) => rsx!(
                video {
                    class: "{message__content__video}",
                    src: "{url}",
                    controls: true,
                    autoplay: false
                }
            ),

            Some(ImageType::Media(content)) => {
                match vec_to_url(content.to_vec()) {
                    Ok(url) => rsx!(
                        video {
                            class: "{message__content__video}",
                            src: "{url}",
                            controls: true,
                            autoplay: false
                        }
                    ),
                    Err(_) => rsx!( strong { "Unable to read file" } ),
                }
            }
            None => rsx!( strong { "File Not Found" } ),
        }
    } else {
        rsx!( File { body: props.body.clone(), is_reply: props.is_reply } )
    }
}
