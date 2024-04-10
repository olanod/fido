use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use futures::TryFutureExt;

use crate::components::atoms::{ChatConversation, Icon, LogOut, MenuItem, UserCircle};
use crate::hooks::use_auth::use_auth;
use crate::hooks::use_auth::LogoutError;
use crate::hooks::use_client::use_client;
use crate::hooks::use_notification::use_notification;
use crate::hooks::use_session::use_session;
use crate::pages::route::Route;

#[derive(PartialEq, Props, Clone)]
pub struct MenuProps {
    on_click: EventHandler<MouseEvent>,
}

pub fn Menu(props: MenuProps) -> Element {
    let i18 = use_i18();
    let nav = use_navigator();
    let mut client = use_client();
    let mut auth = use_auth();
    let session = use_session();
    let mut notification = use_notification();

    let on_log_out = move |_| {
        spawn({
            async move { 
                auth.logout(&mut client, session.is_guest()).await }.unwrap_or_else(
                move |e: LogoutError| {
                    let message = match e {
                        LogoutError::Failed | LogoutError::DefaultClient => translate!(i18, "logout.error.server"),
                        LogoutError::RemoveSession => translate!(i18, "logout.chat.common.error.default_server"),
                    };

                    notification.handle_error(&message)
                },
            )
        });
    };

    rsx! {
        div { class: "menu fade-in-left",
            div { class: "menu__content",
                ul {
                    if !session.is_guest() {
                        li {
                            MenuItem {
                                title: translate!(i18, "menu.profile"),
                                icon: rsx!(Icon { height : 24, width : 24, stroke : "var(--text-1)", icon : UserCircle }),
                                on_click: move |event| {
                                    props.on_click.call(event);
                                    nav.push(Route::Profile {});
                                }
                            }
                        }
                    }

                    li {
                        MenuItem {
                            title: translate!(i18, "menu.chats"),
                            icon: rsx!(
                                Icon { height : 24, width : 24, stroke : "var(--text-1)", icon : ChatConversation
                                }
                            ),
                            on_click: move |event| {
                                props.on_click.call(event);
                                nav.push(Route::ChatList {});
                            }
                        }
                    }
                }
                ul {
                    li {
                        MenuItem {
                            title: translate!(i18, "menu.log_out"),
                            icon: rsx!(Icon { height : 24, width : 24, stroke : "var(--text-1)", icon : LogOut }),
                            on_click: on_log_out
                        }
                    }
                }
            }
        }
    }
}
