use dioxus::prelude::*;

use crate::{
    components::atoms::{Attachment, Icon},
    services::matrix::matrix::FileContent,
    utils::nice_bytes::nice_bytes,
};

#[derive(PartialEq, Props, Clone)]
pub struct FileProps {
    body: FileContent,
    is_reply: bool,
}

pub fn File(props: FileProps) -> Element {
    rsx!(
        section { class: "file",
            div { class: "file__content",
                Icon { stroke: "var(--icon-white)", icon: Attachment }

                span { class: "file__description", "{props.body.body}" }
                {
                    props.body.size.and_then(|size| {
                        let size = nice_bytes(size as f64);
                
                        rsx!(
                            span {
                                class: "file__description",
                                "{size}"
                            }
                        )
                    })
                }
            }
        }
    )
}
