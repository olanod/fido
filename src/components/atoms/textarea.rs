use crate::components::atoms::{icon::Icon, Send};
use dioxus::{html::input_data::keyboard_types, prelude::*};
use wasm_bindgen::JsCast;

#[derive(PartialEq, Props, Clone)]
pub struct TextareaInputProps {
    value: String,
    placeholder: String,
    label: Option<String>,
    on_input: EventHandler<FormEvent>,
    on_keypress: EventHandler<KeyboardEvent>,
    on_click: EventHandler<MouseEvent>,
}

pub fn TextareaInput(props: TextareaInputProps) -> Element {
    let mut sent_handled = use_signal(|| false);
    let mut textarea_wrapper_ref = use_signal(|| None);
    let mut textarea_ref = use_signal(|| None);

    rsx!(
        section {
            class: "textarea",
            if let Some(value) = props.label {
                label { class: "input__label", "{value}" }
            }
            div {
                class: "input-wrapper",
                onmounted: move |event| {
                    event.data.downcast::<web_sys::Element>()
                        .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                        .map(|html_element| textarea_wrapper_ref.set(Some(Box::new(html_element.clone()))));
                    },
                textarea {
                    id: "textarea",
                    class: "textarea__wrapper input",
                    value: props.value.clone(),
                    placeholder: "{props.placeholder}",
                    onmounted: move |event| {
                        event.data.downcast::<web_sys::Element>()
                            .and_then(|element| element.clone().dyn_into::<web_sys::HtmlElement>().ok())
                            .map(|html_element| textarea_ref.set(Some(Box::new(html_element.clone()))));
                    },
                    oninput: move |event| {
                        if !*sent_handled.read() {
                            props.on_input.call(event);
                        }

                        if let Some(textarea_wrapper_element) = textarea_wrapper_ref() {
                            if let Some(textarea_element) = textarea_ref() {
                                if props.value.is_empty() {
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
                            props.on_keypress.call(event)
                        }
                    }
                }
                if !props.value.trim().is_empty() {
                    button {
                        class: "textarea__cta input__cta",
                        onclick: move |event| props.on_click.call(event),
                        Icon {
                            stroke: "var(--icon-subdued)",
                            icon: Send,
                            height: 20,
                            width: 20
                        }
                    }
                }
            }
        }
    )
}
