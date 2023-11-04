use dioxus::prelude::*;

use crate::{
    components::atoms::{Attachment, Icon},
    services::matrix::matrix::FileContent,
};

pub enum Size {
    BYTES(u32),
    KB(u32),
    MB(u32),
    GB(u32),
}

#[derive(PartialEq, Props)]
pub struct FileProps {
    body: FileContent,
}

pub fn File(cx: Scope<FileProps>) -> Element {
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
                    style: "color: var(--text-1)",
                    "{cx.props.body.body}"
                }
                if let Some(size) = cx.props.body.size {
                    let size = nice_bytes(size as f64);

                    rsx!(
                        span {
                            style: "color: var(--text-1)",
                            "{size}"
                        }
                    )
                }
            }
        }
    ))
}

fn nice_bytes(x: f64) -> String {
    let units = vec![
        "bytes", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB",
    ];
    let mut l = 0;
    let mut n = x;

    while n >= 1024.0 && l < units.len() - 1 {
        n /= 1024.0;
        l += 1;
    }

    format!(
        "{:.1} {}",
        n,
        if n < 10.0 && l > 0 {
            units[l]
        } else {
            units[l]
        }
    )
}
