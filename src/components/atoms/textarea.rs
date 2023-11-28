use dioxus::{prelude::*, html::input_data::keyboard_types};
use crate::{
    components::{atoms::{icon::Icon, Send}, molecules::input_message::ReplyingTo},
    utils::get_element::GetElement
};

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
    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();
    let sent_handled = use_ref(cx, || false);

    cx.render(rsx!(
        section {
            style: r#"
                display: flex;
                flex-direction: column;
                gap: 4px;
                width: 100%
            "#,
            if let Some(value) = cx.props.label {
                rsx!(
                    label {
                        class: "input__label",
                        "{value}"
                    }
                )
            }
            div {
                id: "textarea-wrapper",
                class: "input-wrapper",
                textarea {
                    id: "textarea",
                    style: "
                        resize: none;
                        overflow: hidden;
                        height: 20px;
                    ",
                    class: "input",
                    value: cx.props.value,
                    placeholder: "{cx.props.placeholder}",
                    oninput: move |event| {
                        if !*sent_handled.read() {
                            cx.props.on_input.call(event);
                        } 

                        let textarea_wrapper_element = GetElement::<web_sys::HtmlElement>::get_element_by_id("textarea-wrapper");
                        let textarea_element = GetElement::<web_sys::HtmlElement>::get_element_by_id("textarea");

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
                button {
                    class: "input__cta",
                    style: "height: 20px",
                    onclick: move |event| cx.props.on_click.call(event),
                    Icon {
                        stroke: "var(--icon-subdued)",
                        icon: Send,
                        height: 20,
                        width: 20
                    }
                }
            }
        }
    ))
}
