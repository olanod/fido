use dioxus::prelude::*;

#[inline_props]
pub fn SpaceSkeleton(cx: Scope, size: u8) -> Element {
    let size = format!("height: {}px; width: {}px;", size, size);

    render!(rsx!(
        button {
            class: "button button--tertiary padding-reset skeleton",
            div{
                class: "avatar avatar--round avatar--skeleton skeleton",
                style: "{size}",
            }
        }
    ))
}
