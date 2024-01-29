use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct HtmlProps<'a> {
    body: &'a String,
    is_reply: bool,
}

pub fn HtmlMessage<'a>(cx: Scope<'a, HtmlProps<'a>>) -> Element<'a> {
    let message_content_html = if cx.props.is_reply {
        "message__content__html--is-replying message-reply__content--html"
    } else {
        ""
    };

    render!(rsx!(div {
        class: "{message_content_html}",
        dangerous_inner_html: "{cx.props.body}"
    }))
}
