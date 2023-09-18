use dioxus::prelude::*;
use gloo::file::ObjectUrl;

use crate::utils::get_element::GetElement;

#[derive(Clone)]
pub struct AttachFile {
    pub name: String,
    pub preview_url: ObjectUrl,
    pub data: Vec<u8>,
}

#[allow(clippy::needless_return)]
pub fn use_attach(cx: &ScopeState) -> &UseAttachState {
    let attach = use_shared_state::<Option<AttachFile>>(cx).expect("Attach file not provided");

    cx.use_hook(move || UseAttachState {
        inner: attach.clone(),
    })
}

#[derive(Clone)]
pub struct UseAttachState {
    inner: UseSharedState<Option<AttachFile>>,
}

impl UseAttachState {
    pub fn get(&self) -> Option<AttachFile> {
        self.inner.read().as_ref().cloned()
    }

    pub fn set(&self, value: Option<AttachFile>) {
        let mut inner = self.inner.write();
        *inner = value;
    }

    pub fn get_file(&self) -> ObjectUrl {
        let attach_read = self.inner.read().as_ref().cloned();
        let attach_file = attach_read.unwrap();

        attach_file.preview_url
    }

    pub fn reset(&self) {
        let element = GetElement::<web_sys::HtmlInputElement>::get_element_by_id("input_file");

        element.set_files(None);
        element.set_value("");

        self.set(None)
    }
}
