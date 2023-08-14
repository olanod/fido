use dioxus::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::atoms::{Attachment, Icon};

#[derive(Debug)]
pub struct AttachEvent {
    pub value: Vec<u8>,
}

#[derive(Props)]
pub struct AttachProps<'a> {
    on_click: EventHandler<'a, AttachEvent>,
}

pub fn Attach<'a>(cx: Scope<'a, AttachProps<'a>>) -> Element<'a> {
    let button_style = r#"
        cursor: pointer;
        background: var(--surface-3);
        border: none;
        border-radius: 100%;
        width: 2.625rem;
        height: 2.625rem;
    "#;

    cx.render(rsx!(
        button {
            style: "{button_style}",
            onclick: move |_| {
                let window = web_sys::window().expect("global window does not exists");
                let document = window.document().expect("expecting a document on window");
                let val = document.get_element_by_id("input_file").unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap(); 
                val.click();

            } ,
            Icon {
                stroke: "#818898",
                icon: Attachment
            }
        }
    ))
}
