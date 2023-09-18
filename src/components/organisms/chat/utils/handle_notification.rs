use crate::{hooks::use_notification::UseNotificationState, pages::chat::chat::NotificationItem};

pub fn handle_notification(item: NotificationItem, notification: UseNotificationState) {
    notification.set(item);

    let notification = notification.to_owned();

    gloo::timers::callback::Timeout::new(3000, move || {
        notification.set(NotificationItem {
            title: String::from(""),
            body: String::from(""),
            show: false,
        });
    })
    .forget();
}
