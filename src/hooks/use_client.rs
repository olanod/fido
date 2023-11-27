use std::ops::Deref;

use dioxus::prelude::*;
use matrix_sdk::Client;

use crate::MatrixClientState;

#[allow(clippy::needless_return)]
pub fn use_client(cx: &ScopeState) -> &UseClientState {
    let matrix = use_shared_state::<MatrixClientState>(cx).expect("Matrix client not provided");

    cx.use_hook(move || UseClientState {
        inner: matrix.clone(),
    })
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
