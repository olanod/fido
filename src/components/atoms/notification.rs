use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    title: String,
    body: String,
    on_click: EventHandler<MouseEvent>,
}

pub fn Notification(props: ButtonProps) -> Element {
    rsx!(
        button {
            class: "notification",
            onclick: move |event| props.on_click.call(event),
            h3 { class: "notification__title", "{props.title}" }

            p { class: "notification__body", "{props.body}" }
        }
    )
}
