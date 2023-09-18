use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

use crate::{
    components::{
        atoms::room::RoomItem,
        molecules::{
            rooms::{CurrentRoom, FormRoomEvent},
            RoomsList,
        },
    },
    hooks::use_client::use_client,
    pages::route::Route,
    services::matrix::matrix::list_rooms,
};

#[inline_props]
pub fn ChatList(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let client = use_client(cx).get();
    let current_room = use_shared_state::<CurrentRoom>(cx).unwrap();
    let rooms = use_state::<Vec<RoomItem>>(cx, || Vec::new());

    let options_style = r#"
        padding: 10px 0 10px;
    "#;

    let on_click_room = move |evt: FormRoomEvent| {
        nav.push(Route::ChatRoom {
            name: evt.room.id.clone(),
        });
        *current_room.write() = evt.room;
    };

    use_coroutine(cx, |_: UnboundedReceiver<bool>| {
        to_owned![client, rooms];

        async move {
            let rooms_vec = list_rooms(&client).await;

            rooms.set(rooms_vec);
        }
    });

    render! {
        section {
            style: "{options_style}",
            class: "options",
            if rooms.len() > 0 {
                rsx!(
                    RoomsList {
                        rooms: rooms,
                        on_submit: on_click_room
                    }
                )
            }
        }
    }
}
