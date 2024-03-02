use dioxus::prelude::*;

use crate::{
    components::atoms::{Attachment, Icon},
    services::matrix::matrix::FileContent,
    utils::nice_bytes::nice_bytes,
};

#[derive(PartialEq, Props)]
pub struct FileProps {
    body: FileContent,
    is_reply: bool,
}

pub fn File<'a>(cx: Scope<'a, FileProps>) -> Element<'a> {
    cx.render(rsx!(
        section {
            class: "file",
            div {
                class: "file__content",
                Icon {
                    stroke: "var(--icon-white)",
                    icon: Attachment
                }

                span {
                    class: "file__description",
                    "{cx.props.body.body}"
                }
                if let Some(size) = cx.props.body.size {
                    let size = nice_bytes(size as f64);

                    rsx!(
                        span {
                            class: "file__description",
                            "{size}"
                        }
                    )
                }
            }
        }
    ))
}
