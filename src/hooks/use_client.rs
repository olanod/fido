use dioxus::prelude::*;

use matrix_sdk::Client;
use std::ops::Deref;

use crate::{
    services::matrix::matrix::create_client, utils::get_homeserver::Homeserver, MatrixClientState,
};

pub fn use_client() -> UseClientState {
    let matrix = consume_context::<Signal<MatrixClientState>>();

    use_hook(move || UseClientState { inner: matrix })
}

#[derive(Clone, Copy)]
pub struct UseClientState {
    inner: Signal<MatrixClientState>,
}

impl UseClientState {
    pub fn get(&self) -> Client {
        self.inner
            .read()
            .deref()
            .client
            .clone()
            .expect("Client not provided")
    }

    pub fn set(&mut self, client: MatrixClientState) {
        let mut inner = self.inner.write();
        *inner = client;
    }

    pub async fn default(&mut self) -> Result<(), ClientError> {
        let homeserver = Homeserver::new().map_err(|_| ClientError::InvalidUrl)?;

        let c = match create_client(&homeserver.get_base_url()).await {
            Ok(c) => c,
            Err(_) => create_client(&Homeserver::default().get_base_url())
                .await
                .map_err(|_| ClientError::DefaultServer)?,
        };

        self.set(MatrixClientState { client: Some(c) });

        Ok(())
    }
}

pub enum ClientError {
    InvalidUrl,
    DefaultServer,
}
