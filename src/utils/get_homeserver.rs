use url::Url;
use web_sys::{window, UrlSearchParams};

#[derive(Clone, Debug)]
pub struct Homeserver {
    base_url: String,
}

pub enum HomeserverError {
    InvalidParam,
    InvalidHost,
}

impl Homeserver {
    pub fn new() -> Result<Self, HomeserverError> {
        let Some(from_param) = from_homeserver_param() else {
            let Some(from_location) = from_current_host() else {
                return Err(HomeserverError::InvalidHost);
            };

            return Ok(Homeserver {
                base_url: from_location,
            });
        };

        let url = Url::parse(&from_param).map_err(|_| HomeserverError::InvalidParam)?;

        Ok(Homeserver {
            base_url: url.to_string(),
        })
    }

    pub fn get_base_url(&self) -> &str {
        self.base_url.as_str()
    }
}

fn from_current_host() -> Option<String> {
    let w = window()?;
    let protocol = w.location().protocol().ok()?;
    let host = w.location().host().ok()?;

    Some(format!("{}://{}", protocol, host))
}

fn from_homeserver_param() -> Option<String> {
    let search = window()?.location().search().ok()?;

    if search.is_empty() {
        None
    } else {
        let params = UrlSearchParams::new_with_str(&search).ok()?;
        params.get("homeserver")
    }
}
