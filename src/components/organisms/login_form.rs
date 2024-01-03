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

    render! {
        section {
            class: "login-form",
            div{
                class: "login-form__avatar",
                div {
                    class: "login-form__avatar__content",
                    "{cx.props.emoji}"
                }
            }
            h2 {
                class: "login-form__title",
                "{cx.props.title}"
            }
            p {
                class: "login-form__description",
                "{cx.props.description}"
            }

           div {
                class: "login-form__form__head",
                &cx.props.body

                if let Some(error) = cx.props.error {
                    rsx!(
                        div {
                            class: "login-form__form--error",
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
                class: "login-form__cta--filled",
                Button {
                    text: "{cx.props.button_text}",
                    on_click: move |_| {
                        cx.props.on_handle.call(FormLoginEvent::FilledForm)
                    }
                }
            }

            div {
                class: "login-form__cta--action",
                small {
                    class: "login-form__form__text",
                    if cx.props.clear_data {
                        let i18n_map = i18n_map.clone();
                        auth.get_login_cache().map(|data| {
                            render!(
                                rsx!(
                                    p {
                                        class: "login-form__cta--another",
                                        "{i18n_get_key_value(&i18n_map, key_login_chat_saved_another_user)} {data.username}?"
                                        button {
                                            class: "login-form__form__text login__form__text--color button button--tertiary",
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
                                class: "login-form__form__text login__form__text--color button button--tertiary",
                                onclick: move |_| {
                                        cx.props.on_handle.call(FormLoginEvent::CreateAccount)
                                },
                                "{i18n_get_key_value(&i18n_map, key_onboard_signup_cta)}",
                            }
                        ),
                        BeforeSession::Signup => rsx!(
                            "{i18n_get_key_value(&i18n_map, key_onboard_login_description)}"
                            button {
                                class: "login-form__form__text login__form__text--color button button--tertiary",
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
