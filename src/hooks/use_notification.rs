use dioxus::prelude::*;
use matrix_sdk::encryption::verification::SasVerification;

use crate::pages::route::Route;

#[derive(Debug, Clone)]
pub struct NotificationItem {
    pub title: String,
    pub body: String,
    pub show: bool,
    pub handle: NotificationHandle,
}

#[derive(Debug, Clone)]
pub struct NotificationHandle {
    pub value: NotificationType,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Click,
    AcceptSas(SasVerification, Option<Route>),
    None,
}

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

    pub fn set(&self, item: NotificationItem) {
        let mut inner = self.inner.write();
        *inner = item;
    }
}
