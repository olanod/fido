use dioxus::prelude::*;

#[component]
pub fn SpaceSkeleton(size: u8) -> Element {
    let size = format!("height: {}px; width: {}px;", size, size);

    rsx!(
        button { class: "button button--tertiary padding-reset skeleton", 
            div {
                class: "avatar avatar--round avatar--skeleton skeleton",
                style: "{size}"
            }
        }
    )
}
