use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;

use crate::{
    components::{
        atoms::{
            header_main::{HeaderCallOptions, HeaderEvent},
            HeaderMain,
        },
        molecules::{rooms::CurrentRoom, Menu},
    },
    hooks::{use_client::use_client, use_modal::use_modal},
    pages::route::Route,
    services::matrix::matrix::{account, AccountInfo},
};

pub struct TitleHeaderMain {
    pub title: String,
}

pub fn IndexMenu() -> Element {
    use_context_provider::<Signal<CurrentRoom>>(|| Signal::new(CurrentRoom::default()));
    use_context_provider::<Signal<TitleHeaderMain>>(|| {
        Signal::new(TitleHeaderMain {
            title: String::from("Chats"),
        })
    });

    let mut modal = use_modal();
    let client = use_client();

    let mut show_menu = use_signal(|| false);
    let mut profile = use_signal::<AccountInfo>(|| AccountInfo {
        name: String::from(""),
        avatar_uri: None,
    });

    use_coroutine(|_: UnboundedReceiver<()>| async move {
        let data = account(&client.get()).await;

        profile.set(data);
    });

    let header_event = move |evt: HeaderEvent| match evt.value {
        HeaderCallOptions::CLOSE => {
            let current_value = *show_menu.read();
            show_menu.set(!current_value);
        }
        HeaderCallOptions::EDIT => {
            modal.set_header(Some(profile()));
            modal.show();
        }
    };

    rsx!(
        article {
            HeaderMain { on_event: header_event }

            if *show_menu.read() {
                Menu {
                    on_click: move |_| {
                        let current_value = *show_menu.read();
                        show_menu.set(!current_value);
                    }
                }
            }

            Outlet::<Route> {}
        }
    )
}
