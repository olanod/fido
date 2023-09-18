use dioxus::prelude::*;

use crate::{
    components::atoms::{copy::CopyIcon, Avatar, ChatConversation, Close, Group, Icon, NewChat},
    hooks::use_client::use_client,
    services::matrix::matrix::{account, AccountInfo},
};

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
    let client = use_client(cx);
    let profile = use_state::<AccountInfo>(cx, || AccountInfo {
        name: String::from("XXXXXXXXX"),
        avatar_uri: None,
    });

    let container_style = r#"
        position: absolute;
        background: var(--light-modal-backdrop, rgba(0, 0, 0, 0.30));
        height: 100vh;
        width: 100vw;
        top: 0;
    "#;

    let modal_style = r#"
        position: absolute;
        bottom: 0;
        width: 100%;
        background: white;
        padding: 24px 18px;
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
        color: var(--text-loud-900, #0D0D12);
        text-align: center;
        font-family: Inter;
        font-size: 18px;
        font-style: normal;
        font-weight: 600;
        line-height: 24px; /* 133.333% */
        text-align: left;
    "#;

    let message_style = r#"
        color: var(--light-modal-text-secondary, rgba(60, 66, 66, 0.60));
        text-align: center;
        font-variant-numeric: lining-nums proportional-nums;
        
        /* 14/14 - Semibold */
        font-family: SF Pro Rounded;
        font-size: 14px;
        font-style: normal;
        font-weight: 600;
        line-height: 18px; /* 128.571% */
        letter-spacing: 0.6px;
        text-align: left;
    "#;

    let close_style = r#"
        cursor: pointer;
        background: white;
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
    "#;

    let cta_title_style = r#"
        overflow: hidden;
        color: var(--text-loud-900, #0D0D12);
        text-align: center;
        text-overflow: ellipsis;
        whitespace: nowrap;
        
        /* Label/XSmall */
        font-family: Inter;
        font-size: 12px;
        font-style: normal;
        font-weight: 500;
        line-height: 16px; /* 133.333% */
    "#;

    use_coroutine(cx, |_: UnboundedReceiver<bool>| {
        to_owned![client, profile];

        async move {
            let data = account(&client.get()).await;

            profile.set(data);
        }
    });

    cx.render(rsx! {
        section {
            style: "{container_style}",
            div {
                style: "{modal_style}",
                article {
                    style: "{title_container_style}",
                    div {
                        style: "{account_style}",
                        Avatar {
                            name: "{profile.get().name}",
                            size: 36,
                            uri: profile.get().avatar_uri.as_ref()
                        }
                        div {
                            p {
                                style: "{username_style}",
                                "{profile.get().name}, Take the leap"
                            }
                            p {
                                style: "{message_style}",
                                "All it takes is a click :)"
                            }
                        }
                    }
                    button {
                        style: "{close_style}",
                        onclick: move |event| {cx.props.on_close.call(event)},
                        Icon {
                            stroke: "#818898",
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
                            stroke: "#000",
                            icon: NewChat
                        }
                        span {
                            style: "{cta_title_style}",
                            "New Chat"
                        }
                    }
                    button {
                        style: "{cta_style}",
                        onclick: move |_| {
                            cx.props.on_click.call(ModalForm { value: RoomType::GROUP })
                        },
                        Icon {
                            stroke: "#000",
                            icon: Group
                        }
                        span {
                            style: "{cta_title_style}",
                            "New Group"
                        }
                    }
                    button {
                        style: "{cta_style}",
                        onclick: move |_| {
                            cx.props.on_click.call(ModalForm { value: RoomType::CHANNEL })
                        },
                        Icon {
                            stroke: "#000",
                            icon: ChatConversation
                        }
                        span {
                            style: "{cta_title_style}",
                            "Public Channel"
                        }
                    }
                }
            }
        }
    })
}
