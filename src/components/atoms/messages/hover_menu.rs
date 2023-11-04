use dioxus::prelude::*;

use crate::components::atoms::{Icon, Layers, Reply, FileDownload};

#[derive(Debug, Clone)]
pub enum MenuOption {
    Download,
    Reply,
    Close,
    ShowThread,
    CreateThread,
}

#[derive(Debug)]
pub struct MenuEvent {
    pub option: MenuOption,
}

#[derive(Props)]
pub struct HoverMenuProps<'a> {
    options: Vec<MenuOption>,
    on_click: EventHandler<'a, MenuEvent>,
}

pub fn HoverMenu<'a>(cx: Scope<'a, HoverMenuProps<'a>>) -> Element<'a> {
    let option_style = r#"
        border: 1px solid transparent;
        width: 100%;
        text-align: left;
        display: flex;
        align-items: center;
        gap: 4px;
        padding: 8px;
    "#;

    cx.render(rsx!(
        section {
            class: "hover_menu",
            ul {
                for option in &cx.props.options {
                    match option {
                        MenuOption::Reply => {
                            rsx!(
                                li {
                                    button {
                                        style: "{option_style}",
                                        onclick: move |_| {
                                            cx.props.on_click.call(MenuEvent {option: MenuOption::Reply })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: Reply
                                        }
                                        span {
                                            style: "color: var(--text-1)",
                                            "Responder"
                                        }
                                    }
                                }
                            )
                        }
                        MenuOption::ShowThread => {
                            rsx!(
                                li {
                                    button {
                                        style: "{option_style}",
                                        onclick: move |_| {
                                            cx.props.on_click.call(MenuEvent {option: MenuOption::ShowThread })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: Layers
                                        }
                                        span {
                                            style: "color: var(--text-1)",
                                            "Ver hilo"
                                        }
                                    }
                                }
                            )
                        }
                        MenuOption::CreateThread => {
                            rsx!(
                                li {
                                    button {
                                        style: "{option_style}",
                                        onclick: move |_| {
                                            cx.props.on_click.call(MenuEvent {option: MenuOption::CreateThread })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: Layers
                                        }
                                        span {
                                            style: "color: var(--text-1)",
                                            "Crear hilo"
                                        }
                                    }
                                }
                            )
                        }
                        MenuOption::Download => {
                            rsx!(
                                li {
                                    button {
                                        style: "{option_style}",
                                        onclick: move |_| {
                                            cx.props.on_click.call(MenuEvent {option: MenuOption::Download })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: FileDownload
                                        }
                                        span {
                                            style: "color: var(--text-1)",
                                            "Descargar"
                                        }
                                    }
                                }
                            )
                        }
                        MenuOption::Close => {
                            rsx!(div{})
                        }
                    }
                }
            }
        }
    ))
}
