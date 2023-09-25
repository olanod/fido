use dioxus::prelude::*;

use crate::services::matrix::matrix::AccountInfo;

#[allow(clippy::needless_return)]
pub fn use_modal(cx: &ScopeState) -> &UseModalState {
    let modal = use_shared_state::<ModalState>(cx).expect("Modal state not provided");

    cx.use_hook(move || UseModalState {
        inner: modal.clone(),
    })
}

#[derive(Clone)]
pub struct ModalState {
    pub show: bool,
    pub account: Option<AccountInfo>,
}

#[derive(Clone)]
pub struct UseModalState {
    inner: UseSharedState<ModalState>,
}

impl UseModalState {
    pub fn get(&self) -> ModalState {
        self.inner.read().clone()
    }

    pub fn set_header(&self, account: Option<AccountInfo>) {
        let mut inner = self.inner.write();
        inner.account = account;
    }

    pub fn show(&self) {
        let mut inner = self.inner.write();
        inner.show = true;
    }

    pub fn hide(&self) {
        let mut inner = self.inner.write();
        inner.show = false;
    }
}
