use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct TextProps {
    body: String,
    is_reply: bool,
}

pub fn TextMessage(props: TextProps) -> Element {
    rsx!(
        p { class: if props.is_reply { "message-reply__content--text" }, span { "{props.body}" } }
    )
}
