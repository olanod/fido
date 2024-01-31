use std::collections::HashMap;

use crate::MatrixClientState;
use crate::{hooks::use_auth::use_auth, services::matrix::matrix::create_client};
use crate::hooks::use_client::use_client;
use crate::utils::i18n_get_key_value::i18n_get_key_value;
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use gloo::storage::LocalStorage;
use log::info;

use crate::components::atoms::{ChatConversation, Icon, LogOut, MenuItem, UserCircle};

use dioxus_router::prelude::*;

use crate::pages::route::Route;

#[derive(Props)]
pub struct MenuProps<'a> {
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Menu<'a>(cx: Scope<'a, MenuProps<'a>>) -> Element<'a> {
    let i18 = use_i18(cx);
    let i18n_map = HashMap::from([
        ("profile", translate!(i18, "menu.profile")),
        ("chats", translate!(i18, "menu.chats")),
        ("log_out", translate!(i18, "menu.log_out")),
    ]);

    let key_profile = "profile";
    let key_chats = "chats";
    let key_log_out = "log_out";

    let nav = use_navigator(cx);
    let client = use_client(cx);
    let auth = use_auth(cx);

    let log_out = move || {
        cx.spawn({
            to_owned![client, auth];

            async move {
                
                let _ = client.get().logout().await;
                let _ = <LocalStorage as gloo::storage::Storage>::delete("session_file");
                
                let c = create_client("https://matrix.org").await;

            client.set(MatrixClientState {
                client: Some(c.clone()),
            });


                auth.set_logged_in(false)
            }
        });
    };
    
    cx.render(rsx! {
        div {
            class: "menu",
            div {
                class: "menu__content",
                ul {
                    li {
                        MenuItem {
                            title: "{i18n_get_key_value(&i18n_map, key_profile)}",
                            icon: cx.render(rsx!(Icon {height: 24, width: 24, stroke: "var(--text-1)", icon: UserCircle})),
                            on_click: move |event| {
                                cx.props.on_click.call(event);
                                nav.push(Route::Profile {});
                            }
                        }
                     }
    
                     li {
                        MenuItem {
                            title: "{i18n_get_key_value(&i18n_map, key_chats)}",
                            icon: cx.render(rsx!(Icon {height: 24, width: 24, stroke: "var(--text-1)", icon: ChatConversation})),
                            on_click: move |event| {
                                cx.props.on_click.call(event);
                                nav.push(Route::ChatList {});
                            }
                        }
                     }
                }
                ul {
                    li {
                        MenuItem {
                            title: "{i18n_get_key_value(&i18n_map, key_log_out)}",
                            icon: cx.render(rsx!(Icon {height: 24, width: 24, stroke: "var(--text-1)", icon: LogOut})),
                            on_click: move |_| {
                                log_out()
                            }
                        }
                    }
                }
            }
        }
        
    })
}
