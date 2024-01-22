use dioxus::prelude::*;

use crate::components::atoms::{FileDownload, Icon, Layers, Reply};

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
    cx.render(rsx!(
        section {
            class: "hover-menu",
            ul {
                for option in &cx.props.options {
                    match option {
                        MenuOption::Reply => {
                            rsx!(
                                li {
                                    button {
                                        class: "hover-menu__option",
                                        onclick: move |_| {
                                            cx.props.on_click.call(MenuEvent {option: MenuOption::Reply })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: Reply
                                        }
                                        span {
                                            class: "hover-menu__option__title",
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
                                        class: "hover-menu__option",
                                        onclick: move |_| {
                                            cx.props.on_click.call(MenuEvent {option: MenuOption::ShowThread })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: Layers
                                        }
                                        span {
                                            class: "hover-menu__option__title",
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
                                        class: "hover-menu__option",
                                        onclick: move |_| {
                                            cx.props.on_click.call(MenuEvent {option: MenuOption::CreateThread })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: Layers
                                        }
                                        span {
                                            class: "hover-menu__option__title",
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
                                        class: "hover-menu__option",
                                        onclick: move |_| {
                                            cx.props.on_click.call(MenuEvent {option: MenuOption::Download })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: FileDownload
                                        }
                                        span {
                                            class: "hover-menu__option__title",
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
