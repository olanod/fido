use dioxus::prelude::*;

use crate::components::atoms::room::RoomItem;

#[derive(Clone, Debug, PartialEq, Hash, Eq, Default)]
pub struct RoomsList {
    pub public: Vec<RoomItem>,
    pub invited: Vec<RoomItem>,
    pub joined: Vec<RoomItem>,
}

pub fn use_rooms() -> UseRoomsListState {
    let rooms_list = consume_context::<Signal<RoomsList>>();

    use_hook(move || UseRoomsListState { inner: rooms_list })
}

#[derive(Clone, Copy)]
pub struct UseRoomsListState {
    inner: Signal<RoomsList>,
}

impl UseRoomsListState {
    pub fn get(&self) -> RoomsList {
        self.inner.read().clone()
    }

    pub fn get_invited(&self) -> Vec<RoomItem> {
        self.inner.read().invited.clone()
    }

    pub fn get_public(&self) -> Vec<RoomItem> {
        self.inner.read().public.clone()
    }

    pub fn get_joined(&self) -> Vec<RoomItem> {
        self.inner.read().joined.clone()
    }

    pub fn find_invited(&self, id: &str) -> Result<usize, String> {
        let position = self
            .inner
            .read()
            .invited
            .iter()
            .position(|r| r.id.eq(&id))
            .ok_or("Not found".to_string())?;

        Ok(position)
    }

    pub fn find_joined(&self, id: &str) -> Result<usize, String> {
        let position = self
            .inner
            .read()
            .joined
            .iter()
            .position(|r| r.id.eq(&id))
            .ok_or("Not found".to_string())?;

        Ok(position)
    }

    pub fn remove_invited(&mut self, id: &str) -> Result<RoomItem, String> {
        let position = self.find_invited(id)?;
        let room = self.inner.write().invited.remove(position);

        log::info!("room {:?}", room);
        Ok(room)
    }

    pub fn remove_joined(&mut self, id: &str) -> Result<RoomItem, String> {
        let position = self.find_joined(id)?;
        let room = self.inner.write().joined.remove(position);

        log::info!("room {:?}", room);
        Ok(room)
    }

    pub fn push_invited(&mut self, room: RoomItem) {
        let Err(_) = self.find_invited(&room.id) else {
            return;
        };

        let mut inner = self.inner.write();
        inner.invited.push(room)
    }

    pub fn push_joined(&mut self, room: RoomItem) {
        let mut inner = self.inner.write();
        inner.joined.push(room)
    }

    pub fn set(&mut self, room: RoomsList) {
        let mut inner = self.inner.write();
        *inner = room;
    }

    pub fn set_invited(&mut self, rooms: Vec<RoomItem>) {
        let mut inner = self.inner.write();
        inner.invited = rooms;
    }

    pub fn set_public(&mut self, rooms: Vec<RoomItem>) {
        let inner = &mut self.inner.write();
        inner.public = rooms;
    }

    pub fn set_joined(&mut self, rooms: Vec<RoomItem>) {
        let mut inner = self.inner.write();
        inner.joined = rooms;
    }

    pub fn default(&mut self) {
        self.set(RoomsList::default())
    }
}
