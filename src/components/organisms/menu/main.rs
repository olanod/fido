use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;
use log::info;
use matrix_sdk::ruma::{
    api::client::room::{
        create_room::{self},
        Visibility,
    },
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
    hooks::{use_client::use_client, use_modal::use_modal},
    pages::route::Route,
    services::matrix::matrix::{account, AccountInfo},
};

pub fn IndexMenu(cx: Scope) -> Element {
    use_shared_state_provider::<CurrentRoom>(cx, || CurrentRoom {
        id: String::new(),
        name: String::new(),
        avatar_uri: None,
    });

    let profile = use_state::<AccountInfo>(cx, || AccountInfo {
        name: String::from(""),
        avatar_uri: None,
    });

    let modal = use_modal(cx);
    let show_menu = use_ref(cx, || false);
    let client = use_client(cx);

    use_coroutine(cx, |_: UnboundedReceiver<bool>| {
        to_owned![client, profile];

        async move {
            let data = account(&client.get()).await;

            profile.set(data);
        }
    });

    let header_event = move |evt: HeaderEvent| {
        to_owned![show_menu, modal];

        match evt.value {
            HeaderCallOptions::CLOSE => {
                let current_value = *show_menu.read();
                show_menu.set(!current_value);
            }
            HeaderCallOptions::EDIT => {
                modal.set_header(Some(profile.get().clone()));
                modal.show();
            }
        }
    };

    let handle = move || {
        cx.spawn({
            to_owned![client];

            async move {}
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

            Outlet::<Route> {}
        }
    ))
}
