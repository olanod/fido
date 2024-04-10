use dioxus::prelude::*;

use crate::components::molecules::rooms::CurrentRoom;

#[derive(Clone, Debug, PartialEq, Hash, Eq, Default)]
pub enum PreviewRoom {
    Invited(CurrentRoom),
    Creating(CurrentRoom),
    Joining(CurrentRoom),
    #[default]
    None,
}

impl PreviewRoom {
    pub fn is_none(self) -> bool {
        match self {
            PreviewRoom::None => true,
            _ => false,
        }
    }
}

pub fn use_room_preview() -> UseRoomState {
    let preview_room = consume_context::<Signal<PreviewRoom>>();

    use_hook(move || UseRoomState {
        inner: preview_room,
    })
}

#[derive(Clone, Copy)]
pub struct UseRoomState {
    inner: Signal<PreviewRoom>,
}

impl UseRoomState {
    pub fn get(&self) -> PreviewRoom {
        self.inner.read().clone()
    }

    pub fn set(&mut self, room: PreviewRoom) {
        let mut inner = self.inner.write();
        *inner = room;
    }

    pub fn default(&mut self) {
        self.set(PreviewRoom::default())
    }
}
