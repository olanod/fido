use std::ops::Deref;

use dioxus::prelude::*;
use matrix_sdk::Client;

use crate::{components::organisms::login_old::LoggedIn, MatrixClientState};

#[allow(clippy::needless_return)]
pub fn use_init_app(cx: &ScopeState) {
    use_shared_state_provider::<LoggedIn>(cx, || LoggedIn {
        is_logged_in: false,
    });
    use_shared_state_provider::<MatrixClientState>(cx, || MatrixClientState { client: None });
}

#[derive(Clone)]
pub struct UseClientState {
    inner: UseSharedState<MatrixClientState>,
}

impl UseClientState {
    pub fn get(&self) -> Client {
        self.inner.read().deref().client.clone().unwrap()
    }

    pub fn set(&self, client: MatrixClientState) {
        let mut inner = self.inner.write();
        *inner = client;
    }
}
