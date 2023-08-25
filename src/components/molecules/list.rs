use dioxus::prelude::*;

use crate::components::atoms::{
    message::{Message, Messages},
    *,
};

#[derive(Props)]
pub struct ListProps<'a> {
    messages: &'a Messages,
}

pub fn List<'a>(cx: Scope<'a, ListProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div{
            class:"messages-list",
            cx.props.messages.iter().map(|message| {
                rsx!(MessageView {
                    key: "{message.id}",
                    message: Message {
                        id: message.id.clone(),
                        display_name: message.display_name.clone(),
                        content: message.content.clone(),
                        avatar_uri: message.avatar_uri.clone(),
                        reply: None,
                        event_id: None,
                        origin: message.origin.clone(),
                        time: message.time.clone(),
                    },
                    is_replying: false,
                    on_event: move |_| {}
                })
            })
        }
    })
}
