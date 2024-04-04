use dioxus::prelude::*;

use crate::components::atoms::Spinner;

#[component]
pub fn LoadingStatus(text: String) -> Element {
    rsx!(
        div { class: "column spinner-dual-ring--center",
            Spinner {}
            p { class: "loading__title", "{text}" }
        }
    )
}
