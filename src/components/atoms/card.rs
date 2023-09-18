use crate::components::atoms::{Close, Icon};
use dioxus::prelude::*;

#[derive(Props)]
pub struct CardProps<'a> {
    file: &'a str,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element<'a> {
    let attach_style = r#"
        width: 100%;
        height: fit-content;
        padding: 16px;
        -webkit-box-shadow: 0px -4px 16px -12px rgba(0,0,0,0.54);
        -moz-box-shadow: 0px -4px 16px -12px rgba(0,0,0,0.54);
        box-shadow: 0px -4px 16px -12px rgba(0,0,0,0.54)
    "#;

    let attach_container_style = r#"
        height: 70px;
        width: 70px;
        position: relative;
    "#;

    let attach_file_style = r#"
        height: 100%;
        width: 100%;
        object-fit: contain;
        border: 0.5px solid #0001;
        position: relative;
        background: #000;
    "#;

    let close_style = r#"
        cursor: pointer;
        background: white;
        -webkit-box-shadow: 0px 0px 30px 0px rgba(0,0,0,0.54);
        -moz-box-shadow: 0px 0px 30px 0px rgba(0,0,0,0.54);
        box-shadow: 0px 0px 30px 0px rgba(0,0,0,0.54);
        position: absolute;
        border: 1px solid transparent;
        right: -5px;
        top: -5px;
        border-radius: 100%;
        padding: 0;
        height: fit-content;
        width: fit-content;
        display: flex;
        justify-content: center;
    "#;

    cx.render(rsx!(
        section {
            style: "{attach_style}",
            onclick: move |event| cx.props.on_click.call(event),
            div {
                style: "{attach_container_style}",
                img {
                    style: "{attach_file_style}",
                    src: "{cx.props.file}"
                }

                button {
                    style: "{close_style}",
                    onclick: move |event| {cx.props.on_click.call(event)},
                    Icon {
                        stroke: "#818898",
                        icon: Close
                    }
                }
            }
        }
    ))
}
