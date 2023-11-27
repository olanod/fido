use dioxus::prelude::*;

use crate::pages::chat::chat::NotificationItem;

#[allow(clippy::needless_return)]
pub fn use_notification(cx: &ScopeState) -> &UseNotificationState {
    let notification = use_shared_state::<NotificationItem>(cx).expect("Notification not provided");

    cx.use_hook(move || UseNotificationState {
        inner: notification.clone(),
    })
}

#[derive(Clone)]
pub struct UseNotificationState {
    inner: UseSharedState<NotificationItem>,
}

impl UseNotificationState {
    pub fn get(&self) -> NotificationItem {
        self.inner.read().clone()
    }

    pub fn set(&self, client: NotificationItem) {
        let mut inner = self.inner.write();
        *inner = client;
    }
}
