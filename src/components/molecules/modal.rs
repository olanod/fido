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

    let container_style = r#"
        position: fixed;
        height: 100vh;
        width: 100vw;
        top: 0;
        left: 0;
    "#;

    let shadow_style = r#"
        position: absolute;
        background: var(--background-shadow);
        height: 100%;
        width: 100%;
        z-index: 10;
    "#;

    let modal_style = r#"
        position: absolute;
        bottom: 0;
        width: 100%;
        background: var(--background);
        padding: 24px 18px 32px;
        border-radius: 28px 28px 0px 0px;
        z-index: 20;
    "#;

    let title_container_style = r#"
        display: flex;
        justify-content: space-between;
        width: 100%;
    "#;

    let account_style = r#"
        width: 100%;
        display: flex;
        gap: 10px
    "#;

    let username_style = r#"
        color: var(--text-1);
        text-align: center;
        font-family: Inter;
        font-size: 18px;
        font-style: normal;
        font-weight: 600;
        line-height: 24px; /* 133.333% */
        text-align: left;
    "#;

    let message_style = r#"
        color: var(--text-2);
        
        font-size: 14px;
        font-style: normal;
        font-weight: 600;
        line-height: 18px; /* 128.571% */
        text-align: left;
    "#;

    let close_style = r#"
        cursor: pointer;
        background: transparent;
        -webkit-box-shadow: 0px 0px 30px 0px rgba(0,0,0,0.54);
        -moz-box-shadow: 0px 0px 30px 0px rgba(0,0,0,0.54);
        box-shadow: 0px 0px 30px 0px rgba(0,0,0,0.54);
        border: 1px solid transparent;
        border-radius: 100%;
        padding: 0;
        height: fit-content;
        width: fit-content;
        display: flex;
        justify-content: center;
    "#;

    let cta_container_style = r#"
        width: 100%;
        display: flex;
        gap: 8px;
        margin-top: 36px;
    "#;

    let cta_style = r#"
        width: 100%;
        display: flex;
        flex-direction: column;
        padding: 2px;
        align-items: center;
        border: 1px solid transparent;
        cursor: pointer;
    "#;

    let cta_title_style = r#"
        overflow: hidden;
        color: var(--text-1);
        text-align: center;
        text-overflow: ellipsis;
        whitespace: nowrap;
        
        /* Label/XSmall */
        font-family: Inter;
        font-size: 12px;
        font-style: normal;
        font-weight: 500;
        line-height: 18px; /* 133.333% */
    "#;

    cx.render(rsx! {
        section {
            style: "{container_style}",
            div {
                style: "{shadow_style}",
                onclick: move |event| {
                    cx.props.on_close.call(event)
                },
            }
            div {
                style: "{modal_style}",
                class: "fadeIn",
                article {
                    style: "{title_container_style}",
                    div {
                        style: "{account_style}",

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
                                        style: "{username_style}",
                                        "{account.name}, {i18n_get_key_value(&i18n_map, key_modal_title)}"
                                    }
                                    p {
                                        style: "{message_style}",
                                        "{i18n_get_key_value(&i18n_map, key_modal_subtitle)}"
                                    }
                                }
                            )
                        }
                    }
                    button {
                        style: "{close_style}",
                        onclick: move |event| {cx.props.on_close.call(event)},
                        Icon {
                            stroke: "var(--icon-subdued)",
                            icon: Close
                        }
                    }
                }
                article {
                    style: "{cta_container_style}",
                    button {
                        style: "{cta_style}",
                        onclick: move |_| {
                            cx.props.on_click.call(ModalForm { value: RoomType::CHAT })
                        },
                        Icon {
                            stroke: "var(--text-1)",
                            icon: NewChat
                        }
                        span {
                            style: "{cta_title_style}",
                            "{i18n_get_key_value(&i18n_map, key_modal_option_dm)}"
                        }
                    }
                    button {
                        style: "{cta_style}",
                        onclick: move |_| {
                            cx.props.on_click.call(ModalForm { value: RoomType::GROUP })
                        },
                        Icon {
                            stroke: "var(--text-1)",
                            icon: Group
                        }
                        span {
                            style: "{cta_title_style}",
                            "{i18n_get_key_value(&i18n_map, key_modal_option_group)}"
                        }
                    }
                    button {
                        style: "{cta_style}",
                        onclick: move |_| {
                            cx.props.on_click.call(ModalForm { value: RoomType::CHANNEL })
                        },
                        Icon {
                            stroke: "var(--text-1)",
                            icon: ChatConversation
                        }
                        span {
                            style: "{cta_title_style}",
                            "{i18n_get_key_value(&i18n_map, key_modal_option_channel)}"
                        }
                    }
                }
            }
        }
    })
}
