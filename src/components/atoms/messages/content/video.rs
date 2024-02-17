use crate::{
    components::atoms::File,
    services::matrix::matrix::{FileContent, ImageType},
    utils::vec_to_url::vec_to_url,
};
use dioxus::prelude::*;

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

    render!(if !cx.props.is_reply {
        match cx.props.body.source.as_ref() {
            Some(source) => match source {
                ImageType::URL(url) => {
                    rsx!(video {
                        class: "{message__content__video}",
                        src: "{url}",
                        controls: true,
                        autoplay: false
                    })
                }
                ImageType::Media(content) => {
                    let url = vec_to_url(content.to_vec());

                    match url {
                        Ok(url) => rsx!(video {
                            class: "{message__content__video}",
                            src: "{url}",
                            controls: true,
                            autoplay: false
                        }),
                        Err(_) => rsx!(
                            strong {
                                "Unable to read file"
                            }
                        ),
                    }
                }
            },
            None => rsx!(
                strong {
                    "File Not Found"
                }
            ),
        }
    } else {
        rsx!(File {
            body: cx.props.body.clone(),
            is_reply: cx.props.is_reply,
        })
    })
}
