use dioxus::prelude::*;
use log::info;
use matrix_sdk::Client;
use url::Url;

use crate::pages::login::LoggedIn;

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

    let auth_info = use_ref::<LoginInfoBuilder>(cx, || LoginInfoBuilder::new());
    let error = use_state(cx, || None);

    cx.use_hook(move || UseAuthState {
        data: auth_info.clone(),
        error: error.clone(),
        logged_in: logged_in.clone(),
    })
}

#[derive(Clone)]
pub struct UseAuthState {
    data: UseRef<LoginInfoBuilder>,
    error: UseState<Option<String>>,
    logged_in: UseSharedState<LoggedIn>,
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
    }

    pub fn build(&self) -> Result<LoginInfo, &str> {
        info!("{:?}", self.data.read());
        self.data.read().clone().build()
    }

    pub fn is_logged_in(&self) -> LoggedIn {
        self.logged_in.read().clone()
    }

    pub fn set_logged_in(&self, option: bool) {
        *self.logged_in.write() = LoggedIn(option);
    }
}
