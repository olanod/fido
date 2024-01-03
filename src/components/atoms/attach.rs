use dioxus::prelude::*;

use crate::{
    components::atoms::{Attachment, Icon},
    utils::get_element::GetElement,
};

pub enum AttachType<'a> {
    Button,
    Avatar(Element<'a>),
}

#[derive(Debug)]
pub struct AttachEvent {
    pub value: Vec<u8>,
}

#[derive(Props)]
pub struct AttachProps<'a> {
    #[props(default = AttachType::Button)]
    atype: AttachType<'a>,
    on_click: EventHandler<'a, Event<FormData>>,
}

pub fn Attach<'a>(cx: Scope<'a, AttachProps<'a>>) -> Element<'a> {
    let on_handle_attach = move |_| {
        let element = GetElement::<web_sys::HtmlInputElement>::get_element_by_id("input_file");

        element.click();
    };

    cx.render(rsx!(
        match &cx.props.atype {
            AttachType::Button =>
                rsx!(
                    button {
                        class: "attach attach--button",
                        onclick: on_handle_attach,
                        Icon {
                            stroke: "var(--icon-white)",
                            icon: Attachment
                        }
                    }
                ),
            AttachType::Avatar(element) =>
                rsx!(
                    button {
                        class: "attach attach--avatar",
                        onclick: on_handle_attach,
                        element
                    }
                ),
        }

        input {
            r#type: "file",
            id: "input_file",
            class: "attach__input",
            oninput: move |event| cx.props.on_click.call(event)
        }
    ))
}
