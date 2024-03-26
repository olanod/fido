use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{Button, Icon, Warning},
    hooks::{use_auth::use_auth, use_init_app::BeforeSession},
};

pub enum FormLoginEvent {
    CreateAccount,
    Login,
    FilledForm,
    ClearData,
    Guest,
}

#[derive(PartialEq, Props, Clone)]
pub struct LoginFormProps {
    title: String,
    description: String,
    button_text: String,
    emoji: String,
    #[props(!optional)]
    error: Option<String>,
    body: Element,
    #[props(default = false)]
    clear_data: bool,
    on_handle: EventHandler<FormLoginEvent>,
    #[props(!optional)]
    status: Option<String>,
}

pub fn LoginForm(props: LoginFormProps) -> Element {
    let i18 = use_i18();
    let auth = use_auth();

    let before_session = consume_context::<Signal<BeforeSession>>();

    rsx! {
        section {
            class: "login-form",
            div{
                class: "login-form__avatar",
                div {
                    class: "login-form__avatar__content",
                    "{props.emoji}"
                }
            }
            h2 {
                class: "login-form__title",
                "{props.title}"
            }
            p {
                class: "login-form__description",
                "{props.description}"
            }

            div {
                class: "login-form__form__head",
                {props.body}

                if let Some(error) = props.error {
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
                }
            }

            div {
                class: "login-form__cta--filled",
                Button {
                    text: "{props.button_text}",
                    status: props.status.clone(),
                    on_click: move |_| {
                        props.on_handle.call(FormLoginEvent::FilledForm)
                    }
                }
            }

            div {
                class: "login-form__cta--action",
                small {
                    class: "login-form__form__text",
                    if props.clear_data {
                        {
                            auth.get_login_cache().map(|data| {
                                rsx!(
                                    p {
                                        class: "login-form__cta--another",
                                        {translate!(i18, "onboard.login.user")} " {data.username}?"
                                        button {
                                            class: "login-form__form__text login__form__text--color button button--tertiary",
                                            onclick: move |_| {
                                                props.on_handle.call(FormLoginEvent::ClearData)
                                            },
                                            {translate!(i18, "onboard.login.cta.another")}
                                        }
                                    }
                                )
                            })
                        }
                    }
                    match *before_session.read() {
                        BeforeSession::Login => rsx!(
                            {translate!(i18, "onboard.signup.description")}
                            button {
                                class: "login-form__form__text login__form__text--color button button--tertiary",
                                onclick: move |_| {
                                        props.on_handle.call(FormLoginEvent::CreateAccount)
                                },
                                {translate!(i18, "onboard.signup.cta")},
                            }
                            p {
                                class: "login-form__cta--another",
                                {translate!(i18, "onboard.guest.description")}
                                button {
                                    class: "login-form__form__text login__form__text--color button button--tertiary",
                                    onclick: move |_| {
                                        props.on_handle.call(FormLoginEvent::Guest)
                                    },
                                    {translate!(i18, "onboard.guest.cta")}
                                }
                            }
                            p {
                                class: "login-form__cta--another",
                                translate!(i18, "onboard.guest.description")
                                button {
                                    class: "login-form__form__text login__form__text--color button button--tertiary",
                                    onclick: move |_| {
                                        cx.props.on_handle.call(FormLoginEvent::Guest)
                                    },
                                    translate!(i18, "onboard.guest.cta")
                                }
                            }
                        ),
                        BeforeSession::Signup => rsx!(
                            {translate!(i18, "onboard.login.description")}
                            button {
                                class: "login-form__form__text login__form__text--color button button--tertiary",
                                onclick: move |_| {
                                        props.on_handle.call(FormLoginEvent::Login)
                                },
                                {translate!(i18, "onboard.login.cta")},
                            }
                            p {
                                class: "login-form__cta--another",
                                {translate!(i18, "onboard.guest.description")}
                                button {
                                    class: "login-form__form__text login__form__text--color button button--tertiary",
                                    onclick: move |_| {
                                        props.on_handle.call(FormLoginEvent::Guest)
                                    },
                                    {translate!(i18, "onboard.guest.cta")}
                                }
                            }
                        ),
                        BeforeSession::Guest => rsx!(
                            {translate!(i18, "onboard.login.description")}
                            button {
                                class: "login-form__form__text login__form__text--color button button--tertiary",
                                onclick: move |_| {
                                        props.on_handle.call(FormLoginEvent::Login)
                                },
                                {translate!(i18, "onboard.login.cta")},
                            }
                            p {
                                class: "login-form__cta--another",
                                {translate!(i18, "onboard.signup.description")}
                                button {
                                    class: "login-form__form__text login__form__text--color button button--tertiary",
                                    onclick: move |_| {
                                        props.on_handle.call(FormLoginEvent::CreateAccount)
                                },
                                {translate!(i18, "onboard.signup.cta")},
                            }
                            }
                            p {
                                class: "login-form__cta--another",
                                translate!(i18, "onboard.guest.description")
                                button {
                                    class: "login-form__form__text login__form__text--color button button--tertiary",
                                    onclick: move |_| {
                                        cx.props.on_handle.call(FormLoginEvent::Guest)
                                    },
                                    translate!(i18, "onboard.guest.cta")
                                }
                            }
                        ),
                        BeforeSession::Guest => rsx!(
                            translate!(i18, "onboard.login.description")
                            button {
                                class: "login-form__form__text login__form__text--color button button--tertiary",
                                onclick: move |_| {
                                        cx.props.on_handle.call(FormLoginEvent::Login)
                                },
                                translate!(i18, "onboard.login.cta"),
                            }
                            p {
                                class: "login-form__cta--another",
                                translate!(i18, "onboard.signup.description")
                            button {
                                class: "login-form__form__text login__form__text--color button button--tertiary",
                                onclick: move |_| {
                                        cx.props.on_handle.call(FormLoginEvent::CreateAccount)
                                },
                                translate!(i18, "onboard.signup.cta"),
                            }
                            }
                        )
                    }
                }
            }
        }
    }
}
