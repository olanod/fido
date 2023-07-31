use dioxus::prelude::*;

#[derive(Props)]
pub struct MessageInputProps<'a> {
    itype: Option<&'a str>,
    message: &'a str,
    placeholder: &'a str,
    on_input: EventHandler<'a, FormEvent>,
    on_keypress: EventHandler<'a, KeyboardEvent>,
}

pub fn MessageInput<'a>(cx: Scope<'a, MessageInputProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        input {
            r#type: cx.props.itype.unwrap_or("text"),
            class: "input",
            value: cx.props.message,
            placeholder: "{cx.props.placeholder}",
            oninput: move |event| cx.props.on_input.call(event),
            onkeypress: move |event| cx.props.on_keypress.call(event)
        }
    ))
}
