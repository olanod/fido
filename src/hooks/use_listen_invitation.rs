use dioxus::prelude::*;
use futures::StreamExt;
use matrix_sdk::{room::Room, Client};
use ruma::events::room::member::StrippedRoomMemberEvent;

use crate::services::matrix::matrix::format_invited_room;

use super::{use_client::use_client, use_rooms::use_rooms};

pub fn use_listen_invitation() -> UseListenInvitationState {
    let client = use_client();
    let mut rooms = use_rooms();
    let mut handler_added = use_signal(|| false);

    let task_push_invited = use_coroutine(|mut rx: UnboundedReceiver<Room>| async move {
        while let Some(room) = rx.next().await {
            if let Room::Invited(room) = room {
                let Ok(item) = format_invited_room(&client.get(), room).await else {
                    continue;
                };

                rooms.push_invited(item);
            }
        }
    });

    use_coroutine(|_: UnboundedReceiver<()>| async move {
        if !*handler_added.read() {
            client.get().add_event_handler(
                move |_: StrippedRoomMemberEvent, _: Client, room: Room| {
                    let task_push_invited = task_push_invited.clone();
                    async move { task_push_invited.send(room) }
                },
            );

            handler_added.set(true);
        }
    });
    use_hook(move || UseListenInvitationState {})
}

#[derive(Clone, Copy)]
pub struct UseListenInvitationState {}
