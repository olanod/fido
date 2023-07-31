use matrix_sdk::ruma::MxcUri;

pub struct ImageSize {
    pub width: u16,
    pub height: u16,
}

pub fn mxc_to_https_uri(uri: &MxcUri, size: ImageSize) -> Option<String> {
    let (server, id) = uri.parts().unwrap();

    let uri = format!("https://matrix-client.matrix.org/_matrix/media/r0/thumbnail/{}/{}?width={}&height={}&method=scale", server, id, size.width, size.height);
    Some(String::from(uri))
}
