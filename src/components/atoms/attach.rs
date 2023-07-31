use dioxus::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug)]
pub struct AttachEvent {
    pub value: Vec<u8>,
}

#[derive(Props)]
pub struct AttachProps<'a> {
    on_click: EventHandler<'a, AttachEvent>,
}

pub fn Attach<'a>(cx: Scope<'a, AttachProps<'a>>) -> Element<'a> {
    let image_read = use_ref::<Option<Vec<u8>>>(cx, || None);

    let button_style = r#"
        cursor: pointer;
        background: var(--surface-0);
        border: none;
    "#;

    let icon_style = r#"
        fill: white
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
            svg {
              style: "{icon_style}",
              view_box: "0 0 50 50",
              height: 20,
              width: 20,
              path {
                  d: "M 18.55 40.55
                  C 26.18 32.42 34.31 24.84 42.18 16.95
                  C 47.15 11.97 40.62 4.57 35.00 10.01
                  C 28.89 15.91 23.05 22.43 16.73 28.05
                  C 12.52 31.79 16.15 37.18 20.17 33.18
                  Q 27.47 25.91 34.81 18.57
                  A 0.77 0.76 43.0 0 1 35.85 18.54
                  L 36.37 18.98
                  Q 36.88 19.43 36.40 19.91
                  Q 29.78 26.62 23.07 33.25
                  Q 20.63 35.67 19.06 36.25
                  C 14.35 37.99 10.45 31.69 13.81 28.32
                  Q 23.02 19.10 32.22 9.90
                  Q 35.27 6.86 37.20 6.21
                  C 41.66 4.73 46.74 9.31 45.95 13.96
                  Q 45.57 16.19 42.93 18.87
                  Q 24.78 37.31 19.42 42.42
                  C 10.67 50.75 -2.14 38.75 7.88 28.67
                  Q 16.86 19.64 25.79 10.55
                  C 26.32 10.01 27.37 9.56 27.90 10.24
                  A 0.98 0.98 0.0 0 1 27.82 11.53
                  Q 18.75 20.65 9.68 29.68
                  C 0.52 38.79 11.85 47.70 18.55 40.55
                  Z"
              }
            }
        }
    ))
}
