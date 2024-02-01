use dioxus::prelude::*;

use crate::components::molecules::input_message::ReplyingTo;

#[allow(clippy::needless_return)]
pub fn use_reply(cx: &ScopeState) -> &UseReplyState {
    let replying_to =
        use_shared_state::<Option<ReplyingTo>>(cx).expect("Unable to read replying_to");

    cx.use_hook(move || UseReplyState {
        inner: replying_to.clone(),
    })
}

#[derive(Clone)]
pub struct UseReplyState {
    inner: UseSharedState<Option<ReplyingTo>>,
}

impl UseReplyState {
    pub fn get(&self) -> Option<ReplyingTo> {
        self.inner.read().clone()
    }

    pub fn set(&self, replying_to: Option<ReplyingTo>) {
        let mut inner = self.inner.write();
        *inner = replying_to;
    }
}
