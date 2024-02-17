use crate::{
    components::atoms::{Avatar, ChatConversation, Close, Group, Icon, NewChat},
    hooks::use_modal::use_modal,
};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
pub struct ModalForm {
    pub value: RoomType,
}

#[derive(Props)]
pub struct ModalProps<'a> {
    on_click: EventHandler<'a, ModalForm>,
    on_close: EventHandler<'a, MouseEvent>,
}

pub enum RoomType {
    CHAT,
    GROUP,
    CHANNEL,
}

pub fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element<'a> {
    let i18 = use_i18(cx);
    let modal = use_modal(cx);

    cx.render(rsx! {
        section {
            class: "modal",
            div {
                class: "modal__cta--hide",
                onclick: move |event| {
                    cx.props.on_close.call(event)
                },
            }
            div {
                class: "modal__wrapper fadeIn",
                article {
                    class: "modal__title",
                    div {
                        class: "modal__user",
                        if let Some(account) = modal.get().account {
                            rsx!(
                                Avatar {
                                    name: account.name.clone(),
                                    size: 42,
                                    uri: None
                                }
                                div {
                                    p {
                                        class: "modal__user__title",
                                        "{account.name}, " translate!(i18, "modal.title")
                                    }
                                    p {
                                        class: "modal__user__subtitle",
                                        translate!(i18, "modal.subtitle")
                                    }
                                }
                            )
                        }
                    }
                    button {
                        class: "modal__cta--close",
                        onclick: move |event| {cx.props.on_close.call(event)},
                        Icon {
                            stroke: "var(--icon-subdued)",
                            icon: Close
                        }
                    }
                }
                article {
                    class: "modal__cta__container",
                    button {
                        class: "modal__cta__wrapper",
                        onclick: move |_| {
                            cx.props.on_click.call(ModalForm { value: RoomType::CHAT })
                        },
                        Icon {
                            stroke: "var(--text-1)",
                            icon: NewChat
                        }
                        span {
                            class: "modal__cta__title",
                            translate!(i18, "modal.options.dm")
                        }
                    }
                    button {
                        class: "modal__cta__wrapper",
                        onclick: move |_| {
                            cx.props.on_click.call(ModalForm { value: RoomType::GROUP })
                        },
                        Icon {
                            stroke: "var(--text-1)",
                            icon: Group
                        }
                        span {
                            class: "modal__cta__title",
                            translate!(i18, "modal.options.group")
                        }
                    }
                    button {
                        class: "modal__cta__wrapper",
                        onclick: move |_| {
                            cx.props.on_click.call(ModalForm { value: RoomType::CHANNEL })
                        },
                        Icon {
                            stroke: "var(--text-1)",
                            icon: ChatConversation
                        }
                        span {
                            class: "modal__cta__title",
                            translate!(i18, "modal.options.channel")
                        }
                    }
                }
            }
        }
    })
}
