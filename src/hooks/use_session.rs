use dioxus::prelude::*;
use gloo::storage::{errors::StorageError, LocalStorage};
use log::info;
use matrix_sdk::{config::SyncSettings, Client};

use crate::services::matrix::matrix::FullSession;

use super::use_client::use_client;

#[allow(clippy::needless_return)]
pub fn use_session(cx: &ScopeState) -> &UseSessionState {
    let client = use_client(cx);

    cx.use_hook(move || UseSessionState {
        inner: String::new(),
    })
}

#[derive(Clone)]
pub struct UseSessionState {
    inner: String,
}

impl UseSessionState {
    pub async fn sync(
        &self,
        client: Client,
        initial_sync_token: Option<String>,
    ) -> anyhow::Result<()> {
        let mut sync_settings = SyncSettings::default();

        if let Some(sync_token) = initial_sync_token {
            sync_settings = sync_settings.token(sync_token);
        }

        loop {
            match client.sync_once(sync_settings.clone()).await {
                Ok(response) => {
                    Self::persist_sync_token(response.next_batch).await?;
                    break;
                }
                Err(error) => {
                    info!("An error occurred during initial sync: {error}");
                    info!("Trying again from login…");
                }
            }
        }

        info!("The client is ready! Listening to new messages…");

        Ok(())
    }

    async fn persist_sync_token(sync_token: String) -> anyhow::Result<()> {
        let serialized_session: Result<String, StorageError> =
            <LocalStorage as gloo::storage::Storage>::get("session_file");

        let serialized_session = serialized_session.unwrap();
        let mut full_session: FullSession = serde_json::from_str(&serialized_session)?;

        full_session.sync_token = Some(sync_token);
        let serialized_session = serde_json::to_string(&full_session)?;
        let _ = <LocalStorage as gloo::storage::Storage>::set("session_file", serialized_session);

        Ok(())
    }
}
