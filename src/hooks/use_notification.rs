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

pub fn use_notification() -> UseNotificationState {
    let notification = consume_context::<Signal<NotificationItem>>();

    use_hook(move || UseNotificationState {
        inner: notification,
    })
}

#[derive(Clone, Copy)]
pub struct UseNotificationState {
    inner: Signal<NotificationItem>,
}

impl UseNotificationState {
    pub fn get(&self) -> NotificationItem {
        self.inner.read().clone()
    }

    pub fn handle_notification(&mut self, item: NotificationItem) {
        let mut this = self.clone();
        let mut inner = self.inner.clone();
        *inner.write() = item;

        gloo::timers::callback::Timeout::new(3000, move || this.clear()).forget();
    }

    pub fn handle_error(&mut self, body: &str) {
        self.handle_notification(NotificationItem {
            title: String::from("Error"),
            body: String::from(body),
            show: true,
            handle: NotificationHandle {
                value: NotificationType::None,
            },
        });
    }

    pub fn clear(&mut self) {
        let mut inner = self.inner.write();
        *inner = NotificationItem::default();
    }
}
