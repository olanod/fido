use dioxus::prelude::*;
use matrix_sdk::encryption::verification::SasVerification;

use crate::pages::route::Route;

#[derive(Debug, Clone, Default)]
pub struct NotificationItem {
    pub title: String,
    pub body: String,
    pub show: bool,
    pub handle: NotificationHandle,
}

#[derive(Debug, Clone, Default)]
pub struct NotificationHandle {
    pub value: NotificationType,
}

#[derive(Debug, Clone, Default)]
pub enum NotificationType {
    Click,
    AcceptSas(SasVerification, Option<Route>),
    #[default]
    None,
}

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

    pub fn handle_notification(&self, item: NotificationItem) {
        let this = self.clone();
        let inner = self.inner.clone();
        *inner.write() = item;

        gloo::timers::callback::Timeout::new(3000, move || this.clear()).forget();
    }

    pub fn handle_error(&self, body: &str) {
        self.handle_notification(NotificationItem {
            title: String::from("Error"),
            body: String::from(body),
            show: true,
            handle: NotificationHandle {
                value: NotificationType::None,
            },
        });
    }

    pub fn clear(&self) {
        let mut inner = self.inner.write();
        *inner = NotificationItem::default();
    }
}
