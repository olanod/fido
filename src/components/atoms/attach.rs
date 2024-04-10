use dioxus::prelude::*;

use crate::{
    components::atoms::{Attachment, Icon},
    utils::get_element::GetElement,
};

#[derive(PartialEq, Debug, Clone)]
pub enum AttachType {
    Button,
    Avatar(Element),
}

#[derive(PartialEq, Debug, Clone)]
pub struct AttachEvent {
    pub value: Vec<u8>,
}

#[derive(PartialEq, Props, Clone)]
pub struct AttachProps {
    #[props(default = AttachType::Button)]
    atype: AttachType,
    on_click: EventHandler<Event<FormData>>,
}

pub fn Attach(props: AttachProps) -> Element {
    let on_handle_attach = move |_| {
        let element = GetElement::<web_sys::HtmlInputElement>::get_element_by_id("input_file");

        element.click();
    };

    rsx!(
        match &props.atype {
            AttachType::Button => rsx!(
                button {
                    class: "attach attach--button",
                    onclick: on_handle_attach,
                    Icon {
                        stroke: "var(--icon-white)",
                        icon: Attachment
                    }
                }
            ),
            AttachType::Avatar(element) => rsx!(
                button {
                    class: "attach attach--avatar",
                    onclick: on_handle_attach,
                    {element}
                }
            ),
        },
        input {
            r#type: "file",
            id: "input_file",
            class: "attach__input",
            oninput: move |event| props.on_click.call(event)
        }
    )
}
