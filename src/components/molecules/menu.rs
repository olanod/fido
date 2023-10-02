use std::collections::HashMap;

use crate::{hooks::use_client::use_client, pages::login::LoggedIn};
use crate::utils::i18n_get_key_value::i18n_get_key_value;
use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use gloo::storage::LocalStorage;
use log::info;

use crate::components::{
    atoms::{ChatConversation, Icon, LogOut, MenuItem, UserCircle},
    
};

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
    let client = use_client(cx).get();
    let logged_in = use_shared_state::<LoggedIn>(cx).unwrap();

    let menu_style = r#"
        width: 100%;
        height: calc(100% - 75px);
        background: #0006;
        z-index: 100;
        position: absolute;
    "#;

    let content_style = r#"
        width: 75%;
        height: 100%;
        background: white;
        z-index: 1000;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    "#;

    let log_out = move || {
        cx.spawn({
            to_owned![client, logged_in];

            async move {
                
                let _ = client.logout().await;
                let _ = <LocalStorage as gloo::storage::Storage>::delete("session_file");
                
                let window = web_sys::window().expect("global window does not exists");
                let x = window.indexed_db();

                match x {
                    Ok(index_db) => {
                        if let Some(db) = index_db {
                            let x = db.delete_database("b");
                            info!("delete: {:?}", x);
                            let x = db.delete_database("b::matrix-sdk-crypto");
                            info!("delete: {:?}", x);
                            let x = db.delete_database("b::matrix-sdk-state");
                            info!("delete: {:?}", x);
                        } 
                    },
                    Err(err) => todo!(),
                }

                logged_in.write().is_logged_in = false;
            }
        });
    };
    
    cx.render(rsx! {
        div {
            style: "{menu_style}",
            div {
                style: "{content_style}",
                ul {
                    li {
                        MenuItem {
                            title: "{i18n_get_key_value(&i18n_map, key_profile)}",
                            icon: cx.render(rsx!(Icon {height: 24, width: 24, stroke: "#000", icon: UserCircle})),
                            on_click: move |event| {
                                cx.props.on_click.call(event);
                                nav.push(Route::Profile {});
                            }
                        }
                     }
    
                     li {
                        MenuItem {
                            title: "{i18n_get_key_value(&i18n_map, key_chats)}",
                            icon: cx.render(rsx!(Icon {height: 24, width: 24, stroke: "#000", icon: ChatConversation})),
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
                            icon: cx.render(rsx!(Icon {height: 24, width: 24, stroke: "#000", icon: LogOut})),
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
