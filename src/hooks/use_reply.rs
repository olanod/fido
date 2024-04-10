use dioxus::prelude::*;

use crate::components::molecules::input_message::ReplyingTo;

pub fn use_reply() -> UseReplyState {
    let replying_to = consume_context::<Signal<Option<ReplyingTo>>>();

    use_hook(move || UseReplyState { inner: replying_to })
}

#[derive(Clone, Copy)]
pub struct UseReplyState {
    inner: Signal<Option<ReplyingTo>>,
}

impl UseReplyState {
    pub fn get(&mut self) -> Option<ReplyingTo> {
        self.inner.read().clone()
    }

    pub fn set(&mut self, replying_to: Option<ReplyingTo>) {
        let mut inner = self.inner.write();
        *inner = replying_to;
    }
}
