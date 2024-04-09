use dioxus::prelude::*;
use dioxus_std::i18n::*;
use dioxus_std::translate;

use crate::components::atoms::{FileDownload, Icon, Layers, Reply};

#[derive(PartialEq, Debug, Clone)]
pub enum MenuOption {
    Download,
    Reply,
    Close,
    ShowThread,
    CreateThread,
}

#[derive(PartialEq, Debug, Clone)]
pub struct MenuEvent {
    pub option: MenuOption,
}

#[derive(PartialEq, Props, Clone)]
pub struct HoverMenuProps {
    options: Vec<MenuOption>,
    on_click: EventHandler<MenuEvent>,
}

pub fn HoverMenu(props: HoverMenuProps) -> Element {
    let i18 = use_i18();

    rsx!(
        section { class: "hover-menu",
            ul {
                for option in &props.options {
                    match option {
                        MenuOption::Reply => {
                            rsx!(
                                li {
                                    button {
                                        class: "hover-menu__option",
                                        onclick: move |_| {
                                            props.on_click.call(MenuEvent {option: MenuOption::Reply })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: Reply
                                        }
                                        span {
                                            class: "hover-menu__option__title",
                                            {translate!(i18, "chat.menu.reply")}
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
                                            props.on_click.call(MenuEvent {option: MenuOption::ShowThread })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: Layers
                                        }
                                        span {
                                            class: "hover-menu__option__title",
                                            {translate!(i18, "chat.menu.see")}
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
                                            props.on_click.call(MenuEvent {option: MenuOption::CreateThread })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: Layers
                                        }
                                        span {
                                            class: "hover-menu__option__title",
                                            {translate!(i18, "chat.menu.create")}
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
                                            props.on_click.call(MenuEvent {option: MenuOption::Download })
                                        },
                                        Icon {
                                            stroke: "var(--text-1)",
                                            icon: FileDownload
                                        }
                                        span {
                                            class: "hover-menu__option__title",
                                            {translate!(i18, "chat.menu.download")}
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
    )
}
