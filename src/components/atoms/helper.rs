use dioxus::prelude::*;
#[derive(PartialEq, Debug, Clone)]
pub struct HelperData {
    pub title: String,
    pub description: String,
    pub subtitle: String,
    pub example: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct HelperProps {
    helper: HelperData,
    on_click: EventHandler<MouseEvent>,
}

pub fn Helper(props: HelperProps) -> Element {
    rsx!(
        section { class: "helper", onclick: move |event| props.on_click.call(event),
            h3 { class: "helper__title", "{props.helper.title}" }
            p { class: "helper__description", "{props.helper.description}" }
            h4 { class: "helper__subtitle", "{props.helper.subtitle}" }
            p { class: "helper__example", "{props.helper.example}" }
        }
    )
}
