use std::collections::HashMap;

use crate::{
    components::atoms::{Avatar, ChatConversation, Close, Group, Icon, NewChat},
    hooks::use_modal::use_modal,
    utils::i18n_get_key_value::i18n_get_key_value,
};
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
pub struct ModalForm {
    pub value: RoomType,
}

#[derive(Props)]
pub struct ModalProps<'a> {
    // account: &'a AccountInfo,
    on_click: EventHandler<'a, ModalForm>,
    on_close: EventHandler<'a, MouseEvent>,
}

pub enum RoomType {
    CHAT,
    GROUP,
    CHANNEL,
}

pub fn Modal<'a>(cx: Scope<'a, ModalProps<'a>>) -> Element<'a> {
    let modal = use_modal(cx);
    let i18 = use_i18(cx);

    let key_modal_title = "modal-title";
    let key_modal_subtitle = "modal-subtitle";
    let key_modal_option_dm = "modal-option-dm";
    let key_modal_option_group = "modal-option-group";
    let key_modal_option_channel = "modal-option-channel";

    let i18n_map = HashMap::from([
        (key_modal_title, translate!(i18, "modal.title")),
        (key_modal_subtitle, translate!(i18, "modal.subtitle")),
        (key_modal_option_dm, translate!(i18, "modal.options.dm")),
        (
            key_modal_option_group,
            translate!(i18, "modal.options.group"),
        ),
        (
            key_modal_option_channel,
            translate!(i18, "modal.options.channel"),
        ),
    ]);

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
                            let i18n_map = i18n_map.clone();
                            rsx!(
                                Avatar {
                                    name: account.name.clone(),
                                    size: 42,
                                    uri: None
                                }
                                div {
                                    p {
                                        class: "modal__user__title",
                                        "{account.name}, {i18n_get_key_value(&i18n_map, key_modal_title)}"
                                    }
                                    p {
                                        class: "modal__user__subtitle",
                                        "{i18n_get_key_value(&i18n_map, key_modal_subtitle)}"
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
                            "{i18n_get_key_value(&i18n_map, key_modal_option_dm)}"
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
                            "{i18n_get_key_value(&i18n_map, key_modal_option_group)}"
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
                            "{i18n_get_key_value(&i18n_map, key_modal_option_channel)}"
                        }
                    }
                }
            }
        }
    })
}
