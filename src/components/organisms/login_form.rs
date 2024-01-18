use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use log::info;

use crate::{
    components::atoms::{Button, Icon, Warning},
    hooks::{use_auth::use_auth, use_init_app::BeforeSession},
    utils::i18n_get_key_value::i18n_get_key_value,
};

pub enum FormLoginEvent {
    CreateAccount,
    Login,
    FilledForm,
    ClearData,
}

#[derive(Props)]
pub struct LoginFormProps<'a> {
    title: &'a str,
    description: &'a str,
    button_text: &'a str,
    emoji: &'a str,
    #[props(!optional)]
    error: Option<&'a String>,
    body: Element<'a>,
    #[props(default = false)]
    clear_data: bool,
    on_handle: EventHandler<'a, FormLoginEvent>,
}

pub fn LoginForm<'a>(cx: Scope<'a, LoginFormProps<'a>>) -> Element<'a> {
    let i18 = use_i18(cx);
    let auth = use_auth(cx);

    let key_onboard_login_description = "onboard-login-description";
    let key_onboard_login_cta = "onboard-login-cta";

    let key_onboard_signup_description = "onboard-signup-description";
    let key_onboard_signup_cta = "onboard-signup-cta";

    let key_login_chat_saved_another_user = "login-chat-saved-another_user";
    let key_login_chat_saved_cta_another = "login-chat-saved-cta-another";

    let i18n_map = HashMap::from([
        (
            key_onboard_login_description,
            translate!(i18, "onboard.login.description"),
        ),
        (key_onboard_login_cta, translate!(i18, "onboard.login.cta")),
        (
            key_onboard_signup_description,
            translate!(i18, "onboard.signup.description"),
        ),
        (
            key_onboard_signup_cta,
            translate!(i18, "onboard.signup.cta"),
        ),
        (
            key_login_chat_saved_another_user,
            translate!(i18, "login.chat_steps.saved.another_user"),
        ),
        (
            key_login_chat_saved_cta_another,
            translate!(i18, "login.chat_steps.saved.cta_another"),
        ),
    ]);

    let before_session =
        use_shared_state::<BeforeSession>(cx).expect("Unable to use before session");

    let page = r#"
        text-align: center;
        width: 100%;
        margin: auto;
        padding: 12px;
        margin-top: 120px;
    "#;
    let avatar_container = r#"
        border-radius: 100px;
        background: var(--secondary-yellow-50);
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
        color: var(--text-1);
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
        color: var(--text-subdued);
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

    let login_style = r#"
        color: var(--text-normal);

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
    let cta_login_style = r#"
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

                if let Some(error) = cx.props.error {
                    let error_style = r#"
                        display: flex;
                        gap: 2px;
                        color: var(--secondary-red-100);
                        font-family: Inter;
                        font-size: 12px;
                        font-style: normal;
                        font-weight: 400;
                        line-height: 16px; 
                        align-items: center;
                        padding-top: 6px;
                        margin: 0 auto;
                    "#;

                    rsx!(
                        div {
                            style: "{error_style}",
                            Icon {
                                stroke: "var(--secondary-red-100)",
                                height: 16,
                                width: 16,
                                icon: Warning
                            }
                            "{error}"
                        }
                    )
                }
           }

            div {
                style: "{button_style}",
                Button {
                    text: "{cx.props.button_text}",
                    on_click: move |_| {
                        cx.props.on_handle.call(FormLoginEvent::FilledForm)
                    }
                }
            }

            div {
                style: "{cta_login_style}",
                small {
                    style: "{login_style}",
                    if cx.props.clear_data {
                        let i18n_map = i18n_map.clone();

                        auth.get_login_cache().map(|data| {
                            render!(
                                rsx!(
                                    p {
                                        style: "margin-bottom: 8px;",
                                        "{i18n_get_key_value(&i18n_map, key_login_chat_saved_another_user)} {data.username}?"
                                        button {
                                            style: "
                                                {login_style}
                                                color: var(--text-1);
                                            ",
                                            class: "button button--tertiary",
                                            onclick: move |_| {
                                                cx.props.on_handle.call(FormLoginEvent::ClearData)
                                            },
                                            "{i18n_get_key_value(&i18n_map, key_login_chat_saved_cta_another)}",
                                        }
                                    }
                                )
                            )
                        })
                    }
                    match *before_session.read() {
                        BeforeSession::Login => rsx!(
                            "{i18n_get_key_value(&i18n_map, key_onboard_signup_description)}"
                            button {
                                style: "
                                    {login_style}
                                    color: var(--text-1);
                                ",
                                class: "button button--tertiary",
                                onclick: move |_| {
                                        cx.props.on_handle.call(FormLoginEvent::CreateAccount)
                                },
                                "{i18n_get_key_value(&i18n_map, key_onboard_signup_cta)}",
                            }
                        ),
                        BeforeSession::Signup => rsx!(
                            "{i18n_get_key_value(&i18n_map, key_onboard_login_description)}"
                            button {
                                style: "
                                    {login_style}
                                    color: var(--text-1);
                                ",
                                class: "button button--tertiary",
                                onclick: move |_| {
                                        cx.props.on_handle.call(FormLoginEvent::Login)
                                },
                                "{i18n_get_key_value(&i18n_map, key_onboard_login_cta)}",
                            }
                        )
                    }
                }
            }
        }
    }
}
