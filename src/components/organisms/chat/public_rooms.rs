use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::{
        atoms::{
            header_main::{HeaderCallOptions, HeaderEvent},
            Header,
        },
        molecules::{rooms::FormRoomEvent, RoomsList},
    },
    hooks::{
        use_messages::use_messages,
        use_public::use_public,
        use_room_preview::{use_room_preview, PreviewRoom},
        use_rooms::use_rooms,
    },
    pages::route::Route,
};

pub enum PreviewRoomError {
    InvalidRoomId,
    InvitationNotFound,
    AcceptFailed,
}

#[derive(PartialEq, Props, Clone)]
pub struct PublicRoomProps {
    on_back: EventHandler<()>,
}
pub fn PublicRooms(props: PublicRoomProps) -> Element {
    let i18 = use_i18();
    let nav = use_navigator();
    let mut preview = use_room_preview();
    let rooms = use_rooms();
    let mut messages = use_messages();
    let mut public = use_public();

    let header_event = move |evt: HeaderEvent| match evt.value {
        HeaderCallOptions::CLOSE => {
            nav.push(Route::ChatList {});
            public.default();
            props.on_back.call(())
        }
        _ => {}
    };

    let on_click_room = move |evt: FormRoomEvent| {
        messages.reset();
        preview.set(PreviewRoom::Joining(evt.room.clone()));
        public.default();
    };

    rsx! {
        div { class: "active-room",
            Header { text: translate!(i18, "chat.public.title"), on_event: header_event }
            RoomsList {
                rooms: rooms.get_public().clone(),
                is_loading: false,
                on_submit: on_click_room,
                wrap: true
            }
        }
    }
}
