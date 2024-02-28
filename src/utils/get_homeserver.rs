use url::Url;
use web_sys::window;

use super::get_param::get_param;

#[derive(Clone, Debug)]
pub struct Homeserver {
    base_url: String,
}

impl Default for Homeserver {
    fn default() -> Self {
        Self {
            base_url: "https://matrix.org".to_string(),
        }
    }
}

pub enum HomeserverError {
    InvalidUrl,
}

impl Homeserver {
    pub fn new() -> Result<Self, HomeserverError> {
        let base_url = from_homeserver_param()
            .unwrap_or(from_current_host().ok_or(HomeserverError::InvalidUrl)?);

        Ok(Homeserver { base_url })
    }

    pub fn get_base_url(&self) -> &str {
        self.base_url.as_str()
    }
}

fn from_current_host() -> Option<String> {
    let window = window()?;
    let protocol = window.location().protocol().ok()?;
    let host = window.location().host().ok()?;

    Some(format!("{}://{}", protocol, host))
}

fn from_homeserver_param() -> Option<String> {
    let param = get_param("homeserver")?;
    let url = Url::parse(&param).ok()?;
    Some(url.to_string())
}
