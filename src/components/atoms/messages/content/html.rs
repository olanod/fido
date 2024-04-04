use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct HtmlProps {
    body: String,
    is_reply: bool,
}

pub fn HtmlMessage(props: HtmlProps) -> Element {
    rsx!(
        div {
            class: if props.is_reply { "message__content__html--is-replying message-reply__content--html" },
            dangerous_inner_html: "{props.body}"
        }
    )
}
