use dioxus::prelude::*;

use crate::components::atoms::Spinner;

#[inline_props]
pub fn LoadingStatus(cx: Scope, text: String) -> Element {
    cx.render({
        rsx!(
            div {
                class: "column spinner-dual-ring--center",
                Spinner {}

                p {
                    style: "color: var(--text-1)",
                    "{text}"
                }
            }
        )
    })
}
