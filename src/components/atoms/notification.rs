use dioxus::prelude::*;

#[derive(Props)]
pub struct ButtonProps<'a> {
    title: &'a str,
    body: &'a str,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Notification<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        button {
          class: "notification",
          onclick: move |event| cx.props.on_click.call(event),
          h3 {
            class: "notification__title",
            "{cx.props.title}"
        }

        p {
            class: "notification__body",
            "{cx.props.body}"
          }
        }
    ))
}
