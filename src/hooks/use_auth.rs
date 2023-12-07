use dioxus::prelude::*;
use gloo::storage::{errors::StorageError, LocalStorage};
use matrix_sdk::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::pages::login::LoggedIn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheLogin {
    pub server: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LoginInfo {
    pub server: Url,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct LoginInfoBuilder {
    pub server: Option<Url>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl LoginInfoBuilder {
    pub fn new() -> Self {
        LoginInfoBuilder {
            server: None,
            username: None,
            password: None,
        }
    }

    pub fn server(&mut self, server: Url) {
        self.server = Some(server);
    }

    pub fn username(&mut self, username: String) {
        self.username = Some(username);
    }

    pub fn password(&mut self, password: String) {
        self.password = Some(password);
    }

    pub fn build(self) -> Result<LoginInfo, &'static str> {
        if self.server.is_none() || self.username.is_none() || self.password.is_none() {
            Err("Can't build LoginInfo, some parameters are missing")
        } else {
            Ok(LoginInfo {
                server: self.server.unwrap(),
                username: self.username.unwrap(),
                password: self.password.unwrap(),
            })
        }
    }
}

#[allow(clippy::needless_return)]
pub fn use_auth(cx: &ScopeState) -> &UseAuthState {
    let logged_in = use_shared_state::<LoggedIn>(cx).unwrap();
    let login_cache =
        use_shared_state::<Option<CacheLogin>>(cx).expect("Unable to read login cache");

    let auth_info = use_ref::<LoginInfoBuilder>(cx, || LoginInfoBuilder::new());
    let error = use_state(cx, || None);

    cx.use_hook(move || UseAuthState {
        data: auth_info.clone(),
        error: error.clone(),
        logged_in: logged_in.clone(),
        login_cache: login_cache.clone(),
    })
}

#[derive(Clone)]
pub struct UseAuthState {
    data: UseRef<LoginInfoBuilder>,
    error: UseState<Option<String>>,
    logged_in: UseSharedState<LoggedIn>,
    login_cache: UseSharedState<Option<CacheLogin>>,
}

#[derive(Clone)]
pub struct UseAuth {
    pub data: LoginInfoBuilder,
    pub error: Option<String>,
    pub logged_in: LoggedIn,
}

impl UseAuthState {
    pub async fn set_server(&self, homeserver: String) {
        let server_parsed =
            if homeserver.starts_with("http://") || homeserver.starts_with("https://") {
                Url::parse(&homeserver)
            } else {
                Url::parse(&format!("https://{homeserver}"))
            };

        match server_parsed {
            Ok(ref server) => {
                let response = Client::builder()
                    .homeserver_url(&server.as_str())
                    .build()
                    .await;

                self.data.with_mut(|l| l.server(server.clone()));

                match response {
                    Ok(_) => {
                        self.error.set(None);
                    }
                    Err(e) => {
                        self.error.set(Some(e.to_string()));
                    }
                }
            }
            Err(e) => self.error.set(Some(e.to_string())),
        }
    }

    pub fn set_username(&self, username: String, parse: bool) {
        let mut username_parse = username;

        if parse {
            if !username_parse.starts_with("@") {
                username_parse = format!("@{}", username_parse);
            }

            if let Some(server) = &self.data.read().server {
                if let Some(domain) = server.domain() {
                    if !username_parse.ends_with(domain) {
                        username_parse = format!("{}:{}", username_parse, domain);
                    }
                }
            }
        }

        self.data.with_mut(|l| {
            l.username(username_parse);
        });
    }

    pub fn set_password(&self, password: String) {
        self.data.with_mut(|l| {
            l.password(password);
        });
    }

    pub fn set_login_cache(&self, data: CacheLogin) {
        *self.login_cache.write() = Some(data)
    }

    pub fn get_login_cache(&self) -> Option<CacheLogin> {
        self.login_cache.read().clone()
    }

    pub fn get(&self) -> UseAuth {
        UseAuth {
            data: self.data.read().clone(),
            error: self.error.get().clone(),
            logged_in: self.logged_in.read().clone(),
        }
    }

    pub fn reset(&self) {
        self.data.set(LoginInfoBuilder::new());
        self.error.set(None);

        <LocalStorage as gloo::storage::Storage>::delete("login_data");
    }

    pub fn build(&self) -> Result<LoginInfo, &str> {
        self.data.read().clone().build()
    }

    pub fn persist_data(&self, data: CacheLogin) -> anyhow::Result<(), StorageError> {
        let serialized_data = serde_json::to_string(&data)?;
        <LocalStorage as gloo::storage::Storage>::set("login_data", serialized_data)
    }

    pub fn get_storage_data(&self) -> anyhow::Result<String, StorageError> {
        <LocalStorage as gloo::storage::Storage>::get("login_data")
    }

    pub fn is_storage_data(&self) -> bool {
        let data = Self::get_storage_data(&self);

        data.is_ok()
    }

    pub fn is_logged_in(&self) -> LoggedIn {
        self.logged_in.read().clone()
    }

    pub fn set_logged_in(&self, option: bool) {
        *self.logged_in.write() = LoggedIn(option);
    }
}
