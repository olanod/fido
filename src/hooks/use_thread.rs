use dioxus::prelude::*;

use crate::services::matrix::matrix::TimelineThread;

pub fn use_thread() -> UseThreadState {
    let replying_to = consume_context::<Signal<Option<TimelineThread>>>();

    use_hook(move || UseThreadState { inner: replying_to })
}

#[derive(Clone, Copy)]
pub struct UseThreadState {
    inner: Signal<Option<TimelineThread>>,
}

impl UseThreadState {
    pub fn get(&self) -> Option<TimelineThread> {
        self.inner.read().clone()
    }

    pub fn set(&mut self, replying_to: Option<TimelineThread>) {
        let mut inner = self.inner.write();
        *inner = replying_to;
    }
}
