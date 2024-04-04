use dioxus::prelude::*;

use crate::services::matrix::matrix::AccountInfo;

pub fn use_modal() -> UseModalState {
    let modal = consume_context::<Signal<ModalState>>();

    use_hook(move || UseModalState { inner: modal })
}

#[derive(Clone)]
pub struct ModalState {
    pub show: bool,
    pub account: Option<AccountInfo>,
}

#[derive(Clone, Copy)]
pub struct UseModalState {
    inner: Signal<ModalState>,
}

impl UseModalState {
    pub fn get(&self) -> ModalState {
        self.inner.read().clone()
    }

    pub fn set_header(&mut self, account: Option<AccountInfo>) {
        let mut inner = self.inner.write();
        inner.account = account;
    }

    pub fn show(&mut self) {
        let mut inner = self.inner.write();
        inner.show = true;
    }

    pub fn hide(&mut self) {
        let mut inner = self.inner.write();
        inner.show = false;
    }
}
