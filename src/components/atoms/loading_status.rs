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
                    class: "loading__title",
                    "{text}"
                }
            }
        )
    })
}
