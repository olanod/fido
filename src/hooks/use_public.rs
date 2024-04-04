use dioxus::prelude::*;

#[derive(Default, Debug, Clone)]
pub struct PublicState {
    pub show: bool,
}

pub fn use_public() -> UsePublicState {
    let public_state = consume_context::<Signal<PublicState>>();

    use_hook(move || UsePublicState {
        inner: public_state,
    })
}

#[derive(Clone, Copy)]
pub struct UsePublicState {
    inner: Signal<PublicState>,
}

impl UsePublicState {
    pub fn get(&self) -> PublicState {
        self.inner.read().clone()
    }

    pub fn set(&mut self, room: PublicState) {
        let mut inner = self.inner.write();
        *inner = room;
    }

    pub fn default(&mut self) {
        self.set(PublicState::default())
    }
}
