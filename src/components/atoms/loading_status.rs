use dioxus::prelude::*;

use crate::components::atoms::Spinner;

#[inline_props]
pub fn LoadingStatus<'a>(cx: Scope<'a>, text: &'a str) -> Element<'a> {
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
