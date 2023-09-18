use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;
use log::info;
use matrix_sdk::ruma::{
    api::client::room::{
        create_room::{self},
        Visibility,
    },
    events::AnyInitialStateEvent,
    user_id, OwnedUserId, UserId,
};

use crate::{
    components::{
        atoms::{
            header_main::{HeaderCallOptions, HeaderEvent},
            HeaderMain,
        },
        molecules::{
            modal::{ModalForm, RoomType},
            rooms::CurrentRoom,
            Menu, Modal,
        },
    },
    hooks::use_client::use_client,
    pages::route::Route,
};

pub fn IndexMenu(cx: Scope) -> Element {
    use_shared_state_provider::<CurrentRoom>(cx, || CurrentRoom {
        id: String::new(),
        name: String::new(),
        avatar_uri: None,
    });

    let show_menu = use_ref(cx, || false);
    let show_modal = use_ref(cx, || false);
    let client = use_client(cx);

    let header_event = move |evt: HeaderEvent| {
        to_owned![show_menu];

        match evt.value {
            HeaderCallOptions::CLOSE => {
                let current_value = *show_menu.read();
                show_menu.set(!current_value);
            }
            HeaderCallOptions::EDIT => {
                let current_value = *show_modal.read();
                show_modal.set(!current_value);
            }
        }
    };

    let handle = move || {
        cx.spawn({
            to_owned![client];

            async move {
                
            }
        })
    };

    cx.render(rsx!(
        article {
            HeaderMain{
                on_event: header_event
            }

            if *show_menu.read() {
                rsx!(
                    Menu {
                        on_click:move |_|{
                            let current_value = *show_menu.read();
                            show_menu.set(!current_value);
                        }
                    }
                )
            }

            if *show_modal.read() {
                rsx!(
                    Modal {
                        on_click: move |event: ModalForm| {
                            match event.value {
                                RoomType::CHAT => {
                                    handle()
                                },
                                RoomType::GROUP => {},
                                RoomType::CHANNEL => {}
                            }
                        },
                        on_close:move |_|{
                            let current_value = *show_modal.read();
                            show_modal.set(!current_value);
                        }
                    }
                )
            }

            Outlet::<Route> {}
        }
    ))
}
