use dioxus::prelude::*;

use crate::components::organisms::chat::ActiveRoom;

// The name prop comes from the /:name route segment
#[inline_props]
pub fn ChatRoom(cx: Scope, name: String) -> Element {
    render! {
        ActiveRoom {}
    }
}
