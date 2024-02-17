use dioxus::prelude::*;

use crate::{components::atoms::message::Messages, services::matrix::matrix::TimelineRelation};

#[allow(clippy::needless_return)]
pub fn use_messages(cx: &ScopeState) -> &UseMessagesState {
    let messages = use_shared_state::<Messages>(cx).expect("Unable to use Messages");

    cx.use_hook(move || UseMessagesState {
        inner: messages.clone(),
    })
}

#[derive(Clone)]
pub struct UseMessagesState {
    inner: UseSharedState<Messages>,
}

impl UseMessagesState {
    pub fn get(&self) -> Messages {
        self.inner.read().clone()
    }

    pub fn set(&self, messages: Messages) {
        let mut inner = self.inner.write();
        *inner = messages;
    }

    pub fn push(&self, message: TimelineRelation) {
        self.inner.write().push(message);
    }

    pub fn reset(&self) {
        self.inner.write().clear();
    }
}
