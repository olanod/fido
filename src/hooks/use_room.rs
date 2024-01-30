use dioxus::prelude::*;

use crate::components::molecules::rooms::CurrentRoom;

#[allow(clippy::needless_return)]
pub fn use_room(cx: &ScopeState) -> &UseRoomState {
    let current_room = use_shared_state::<CurrentRoom>(cx).expect("Unable to use CurrentRoom");

    cx.use_hook(move || UseRoomState {
        inner: current_room.clone(),
    })
}

#[derive(Clone)]
pub struct UseRoomState {
    inner: UseSharedState<CurrentRoom>,
}

impl UseRoomState {
    pub fn get(&self) -> CurrentRoom {
        self.inner.read().clone()
    }

    pub fn set(&self, room: CurrentRoom) {
        let mut inner = self.inner.write();
        *inner = room;
    }
}
