use dioxus::prelude::*;

use crate::components::{atoms::message::Messages, organisms::chat::ActiveRoom};

// The name prop comes from the /:name route segment
#[inline_props]
pub fn ChatRoom(cx: Scope, name: String) -> Element {
    let messages = use_shared_state::<Messages>(cx).expect("Unable to use Messages");

    use_coroutine(cx, |_: UnboundedReceiver<bool>| {
        to_owned![messages];

        async move {
            messages.write().clear();
        }
    });

    render! {
        ActiveRoom {}
    }
}
