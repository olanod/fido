use dioxus::prelude::*;

use crate::components::molecules::rooms::CurrentRoom;

pub fn use_room() -> UseRoomState {
    let current_room = consume_context::<Signal<CurrentRoom>>();

    use_hook(|| UseRoomState {
        inner: current_room,
    })
}

#[derive(Clone, Copy)]
pub struct UseRoomState {
    inner: Signal<CurrentRoom>,
}

impl UseRoomState {
    pub fn get(&self) -> CurrentRoom {
        self.inner.read().clone()
    }

    pub fn set(&mut self, room: CurrentRoom) {
        let mut inner = self.inner.write();
        *inner = room;
    }

    pub fn default(&mut self) {
        self.set(CurrentRoom::default())
    }
}
