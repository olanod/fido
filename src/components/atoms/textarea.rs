use dioxus::{prelude::*, html::input_data::keyboard_types};
use wasm_bindgen::JsCast;
use crate::components::atoms::{icon::Icon, Send};

#[derive(Props)]
pub struct TextareaInputProps<'a> {
    value: &'a str,
    placeholder: &'a str,
    label: Option<&'a str>,
    on_input: EventHandler<'a, FormEvent>,
    on_keypress: EventHandler<'a, KeyboardEvent>,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn TextareaInput<'a>(cx: Scope<'a, TextareaInputProps<'a>>) -> Element<'a> {
    let sent_handled = use_ref(cx, || false);
    let textarea_wrapper_ref = use_ref(cx, || None);
    let textarea_ref = use_ref(cx, || None);

    cx.render(rsx!(
        section {
            class: "textarea",
            if let Some(value) = cx.props.label {
                rsx!(
                    label {
                        class: "input__label",
                        "{value}"
                    }
                )
            }
            div {
                class: "input-wrapper",
                onmounted: move |event| {
                    event.data.get_raw_element()
                        .ok()
                        .and_then(|raw_element| raw_element.downcast_ref::<web_sys::Element>())
                        .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                        .map(|html_element| textarea_wrapper_ref.set(Some(Box::new(html_element.clone()))));
                    },
                textarea {
                    id: "textarea",
                    class: "textarea__wrapper input",
                    value: cx.props.value,
                    placeholder: "{cx.props.placeholder}",
                    onmounted: move |event| {
                        event.data.get_raw_element()
                            .ok()
                            .and_then(|raw_element| raw_element.downcast_ref::<web_sys::Element>())
                            .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                            .map(|html_element| textarea_ref.set(Some(Box::new(html_element.clone()))));
                    },
                    oninput: move |event| {
                        if !*sent_handled.read() {
                            cx.props.on_input.call(event);
                        } 

                        if let Some(textarea_wrapper_element) = textarea_wrapper_ref.read().as_ref() {
                            if let Some(textarea_element) = textarea_ref.read().as_ref() {
                                if cx.props.value.len() == 0 {
                                    textarea_wrapper_element.style().set_css_text(r#""#);
                                    textarea_element.style().set_css_text(r#"
                                        resize: none;
                                        overflow: hidden;
                                        height: 20px;
                                    "#);

                                    sent_handled.set(false);

                                    return
                                } 
                                textarea_element.style().set_css_text(r#"
                                    height: 0px;
                                "#);

                                let scroll_height = textarea_element.scroll_height();
                                
                                // Modify the align if is multiline
                                if scroll_height > 20 {
                                    textarea_wrapper_element.style().set_css_text(r#"
                                        height: auto;
                                        align-items: flex-end;
                                    "#);
                                }

                                // Resize the textarea element 
                                let new_style = if scroll_height < 100 {    
                                format!(r#"
                                        resize: none;
                                        overflow: hidden;
                                        height: {scroll_height}px;
                                    "#)
                                } else {
                                    format!(r#"
                                        resize: none;
                                        height: 100px;
                                    "#)
                                };

                                textarea_element.style().set_css_text(new_style.as_str());
                            }
                        }
                    },
                    onkeypress: move |event| {
                        let modifiers = event.modifiers();

                        match modifiers {
                            keyboard_types::Modifiers::SHIFT => {}
                            _ => {
                                if event.code() == keyboard_types::Code::Enter {
                                    sent_handled.set(true)
                                }
                            }
                        }

                        if modifiers.is_empty() {
                            cx.props.on_keypress.call(event)
                        }
                    }
                }
                if cx.props.value.trim().is_empty() {
                    rsx!(
                        button {
                            class: "textarea__cta input__cta",
                            onclick: move |event| cx.props.on_click.call(event),
                            Icon {
                                stroke: "var(--icon-subdued)",
                                icon: Send,
                                height: 20,
                                width: 20
                            }
                        }
                    )
                }
            }
        }
    ))
}
