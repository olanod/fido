use dioxus::prelude::*;
#[derive(Debug, Clone)]
pub struct HelperData {
    pub title: String,
    pub description: String,
    pub subtitle: String,
    pub example: String,
}

#[derive(Props)]
pub struct HelperProps<'a> {
    helper: HelperData,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Helper<'a>(cx: Scope<'a, HelperProps<'a>>) -> Element<'a> {
    render!(rsx!(
        section {
            class: "helper",
            onclick: move |event| cx.props.on_click.call(event),
            h3 {
                class: "helper__title",
                "{cx.props.helper.title}"
            },
            p {
                class: "helper__description",
                "{cx.props.helper.description}"
            },
            h4 {
                class: "helper__subtitle",
                "{cx.props.helper.subtitle}"
            },
            p {
                class: "helper__example",
                "{cx.props.helper.example}"
            }
        }
    ))
}
