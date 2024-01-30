use matrix_sdk::ruma::MxcUri;

pub struct ImageSize {
    pub width: u16,
    pub height: u16,
}

pub enum ImageMethod {
    CROP,
    SCALE,
}

impl ImageSize {
    pub fn default() -> Self {
        Self::new(48, 48)
    }

    pub fn new(width: u16, height: u16) -> Self {
        ImageSize { width, height }
    }
}

pub fn mxc_to_thumbnail_uri(uri: &MxcUri, size: ImageSize, method: ImageMethod) -> Option<String> {
    match uri.parts() {
        Ok((server, id)) => {
            let method = match method {
                ImageMethod::CROP => "crop",
                ImageMethod::SCALE => "scale",
            };

            let uri = format!("https://matrix-client.matrix.org/_matrix/media/v3/thumbnail/{}/{}?width={}&height={}&method={}", server, id, size.width, size.height, method);
            Some(uri)
        }
        Err(_) => None,
    }
}

pub fn mxc_to_download_uri(uri: &MxcUri) -> Option<String> {
    match uri.parts() {
        Ok((server, id)) => {
            let uri = format!(
                "https://matrix-client.matrix.org/_matrix/media/v3/download/{}/{}",
                server, id
            );
            Some(uri)
        }
        Err(_) => None,
    }
}
