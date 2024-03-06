use dioxus::prelude::*;

use crate::services::matrix::matrix::TimelineThread;

pub fn use_thread(cx: &ScopeState) -> &UseThreadState {
    let replying_to =
        use_shared_state::<Option<TimelineThread>>(cx).expect("Unable to read TimelineThread");

    cx.use_hook(move || UseThreadState {
        inner: replying_to.clone(),
    })
}

#[derive(Clone)]
pub struct UseThreadState {
    inner: UseSharedState<Option<TimelineThread>>,
}

impl UseThreadState {
    pub fn get(&self) -> Option<TimelineThread> {
        self.inner.read().clone()
    }

    pub fn set(&self, replying_to: Option<TimelineThread>) {
        let mut inner = self.inner.write();
        *inner = replying_to;
    }
}
