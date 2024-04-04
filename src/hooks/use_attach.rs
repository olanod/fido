use dioxus::prelude::*;
use gloo::file::ObjectUrl;
use mime::Mime;

use crate::utils::get_element::GetElement;

#[derive(Clone)]
pub struct AttachFile {
    pub name: String,
    pub preview_url: ObjectUrl,
    pub data: Vec<u8>,
    pub content_type: Mime,
    pub size: u64,
}

#[derive(Clone, Debug)]
pub enum AttachError {
    NotFound,
    UncoverType,
    UnknownContent,
}

pub fn use_attach() -> UseAttachState {
    let attach = consume_context::<Signal<Option<AttachFile>>>();

    use_hook(move || UseAttachState { inner: attach })
}

#[derive(Clone, Copy)]
pub struct UseAttachState {
    inner: Signal<Option<AttachFile>>,
}

impl UseAttachState {
    pub fn get(&self) -> Option<AttachFile> {
        self.inner.read().as_ref().cloned()
    }

    pub fn set(&mut self, value: Option<AttachFile>) {
        let mut inner = self.inner.write();
        *inner = value;
    }

    pub fn get_file(&self) -> Result<ObjectUrl, AttachError> {
        let attach_read = self.inner.read().as_ref().cloned();
        match attach_read {
            Some(file) => Ok(file.preview_url),
            None => Err(AttachError::NotFound),
        }
    }

    pub fn reset(&mut self) {
        let element = GetElement::<web_sys::HtmlInputElement>::get_element_by_id("input_file");

        element.set_files(None);
        element.set_value("");

        self.set(None)
    }
}
