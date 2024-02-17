use std::ops::Deref;

use gloo::file::BlobContents;
use wasm_bindgen::JsValue;
use web_sys::Url;

pub fn vec_to_url(content: Vec<u8>) -> Result<String, JsValue> {
    let c = content.deref();
    let mut parts = js_sys::Array::of1(&unsafe { c.into_jsvalue() });
    let blob = web_sys::Blob::new_with_u8_array_sequence(&parts)?;
    let url = Url::create_object_url_with_blob(&blob)?;

    Ok(url)
}
