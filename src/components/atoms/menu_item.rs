use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct MenuItemProps {
    #[props(!optional)]
    title: String,
    icon: Element,
    on_click: EventHandler<MouseEvent>,
}

pub fn MenuItem(props: MenuItemProps) -> Element {
    rsx! {
        div {
            class: "room-view",
            onclick: move |event| props.on_click.call(event),

            {props.icon},
            p { class: "room-view__title", "{props.title}" }
        }
    }
}
