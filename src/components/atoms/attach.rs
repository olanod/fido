use dioxus::prelude::*;

use crate::{
    components::atoms::{Attachment, Avatar, Icon},
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
    let button_style = r#"
        cursor: pointer;
        background: var(--surface-3);
        border: none;
        border-radius: 100%;
        max-width: 2.625rem;
        width: 100%;
        height: 2.625rem;
    "#;

    let avatar_style = r#"
        cursor: pointer;
        background: #E9C8FD;
        border: none;
        border-radius: 100%;
        width: 80px;
        height: 80px;
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 0;
        margin: 0 auto;
    "#;

    let input_attach_style = r#"
        visibility: hidden;
        width: 0;
        display: none;
    "#;

    let on_handle_attach = move |_| {
        let element = GetElement::<web_sys::HtmlInputElement>::get_element_by_id("input_file");

        element.click();
    };

    cx.render(rsx!(
        match &cx.props.atype {
            AttachType::Button => {
                rsx!(
                    button {
                        style: "{button_style}",
                        onclick: on_handle_attach,
                        Icon {
                            stroke: "#fff",
                            icon: Attachment
                        }
                    }
                )
            },
            AttachType::Avatar(element) => {
                rsx!(
                    button {
                        style: "{avatar_style}",
                        onclick: on_handle_attach,
                        element
                    }
                )
            },
        }



        input {
            style: "{input_attach_style}",
            r#type: "file",
            id: "input_file",
            oninput: move |event| cx.props.on_click.call(event)
        }
    ))
}
