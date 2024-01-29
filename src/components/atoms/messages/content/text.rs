use dioxus::prelude::*;

#[derive(PartialEq, Props)]
pub struct TextProps<'a> {
    body: &'a String,
    is_reply: bool,
}

pub fn TextMessage<'a>(cx: Scope<'a, TextProps<'a>>) -> Element<'a> {
    let message_reply = if cx.props.is_reply {
        "message-reply__content--text"
    } else {
        ""
    };

    render!(rsx!(
      p {
        class: "{message_reply}",
        span {
          "{cx.props.body}"
        }
      }
    ))
}
