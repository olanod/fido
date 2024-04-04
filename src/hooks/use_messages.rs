use dioxus::prelude::*;

use crate::{components::atoms::message::Messages, services::matrix::matrix::TimelineRelation};

pub fn use_messages() -> UseMessagesState {
    let messages = consume_context::<Signal<Messages>>();

    use_hook(move || UseMessagesState { inner: messages })
}

#[derive(Clone, Copy)]
pub struct UseMessagesState {
    inner: Signal<Messages>,
}

impl UseMessagesState {
    pub fn get(&self) -> Messages {
        self.inner.read().clone()
    }

    pub fn set(&mut self, messages: Messages) {
        let mut inner = self.inner.write();
        *inner = messages;
    }

    pub fn push(&mut self, message: TimelineRelation) {
        self.inner.write().push(message);
    }

    pub fn reset(&mut self) {
        self.inner.write().clear();
    }
}
