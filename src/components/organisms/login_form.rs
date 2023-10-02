use dioxus::prelude::*;

use crate::components::atoms::Button;

pub struct FormLoginEvent {
    pub value: String,
}

#[derive(Props)]
pub struct LoginFormProps<'a> {
    title: &'a str,
    description: &'a str,
    button_text: &'a str,
    emoji: &'a str,
    // cta: &'a str,
    body: Element<'a>,
    on_handle: EventHandler<'a, FormLoginEvent>,
}

pub fn LoginForm<'a>(cx: Scope<'a, LoginFormProps<'a>>) -> Element<'a> {
    let message_field = use_state(cx, String::new);

    let page = r#"
        text-align: center;
        width: 100%;
        margin: auto;
        padding: 12px;
        margin-top: 120px;
    "#;
    let avatar_container = r#"
        border-radius: 100px;
        background: #FFF5D3;
        display: flex;
        justify-content: center;
        align-items: center;
        margin: auto;
        width: fit-content;
        height: fit-content;
    "#;
    let avatar_style = r#"
        padding: 14px;
        font-size: 33px;
        
    "#;
    let title_style = r#"
        color: var(--text-loud-900, #0D0D12);
        font-family: Inter;
        font-size: 24px;
        font-style: normal;
        font-weight: 500;
        line-height: 32px; /* 133.333% */
        letter-spacing: -0.24px;
        padding-top: 6px;
        margin: auto;
    "#;
    let description_style = r#"
        color: var(--text-subdued-400, #818898);
        text-align: center;
        
        /* Body/Medium */
        font-family: Inter;
        font-size: 16px;
        font-style: normal;
        font-weight: 400;
        line-height: 24px; /* 150% */
        width: 254px;
        margin: auto;
    "#;

    let _login_style = r#"
        color: var(--text-normal-500, #666D80);

        /* Label/Small */
        font-family: Inter;
        font-size: 14px;
        font-style: normal;
        font-weight: 500;
        line-height: 20px; /* 142.857% */
    "#;

    let button_style = r#"
        padding-top: 24px;
    "#;
    let _cta_login_style = r#"
        padding-top: 16px;
    "#;

    render! {
        section {
            style: "{page}",
            div{
                style: "{avatar_container}",
                div {
                    style: "{avatar_style}",
                    "{cx.props.emoji}"
                }
            }
            h2 {
                style: "{title_style}",
                "{cx.props.title}"
            }
            p {
                style: "{description_style}",
                "{cx.props.description}"
            }

           div {
            style: "
                display: flex;
                gap: 16px;
                flex-direction: column;
                padding-top: 36px;
            ",
            &cx.props.body
           }

            div {
                style: "{button_style}",
                Button {
                    text: "{cx.props.button_text}",
                    on_click: move |_event| {
                        message_field.set(String::new());
                        cx.props.on_handle.call(FormLoginEvent {value: message_field.get().to_string()})
                    }
                }
            }

            // div {
            //     style: "{cta_login_style}",
            //     small {
            //         style: "{login_style}",
            //         "Already have an account? Log in"
            //     }
            // }
        }
    }
}
