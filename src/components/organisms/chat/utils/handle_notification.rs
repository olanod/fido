use crate::hooks::use_notification::{UseNotificationState, NotificationItem, NotificationHandle, NotificationType};

pub fn handle_notification(item: NotificationItem, notification: UseNotificationState) {
    notification.set(item);

    let notification = notification.to_owned();

    gloo::timers::callback::Timeout::new(3000, move || {
        notification.set(NotificationItem {
            title: String::from(""),
            body: String::from(""),
            show: false,
            handle: NotificationHandle {
                value: NotificationType::None,
            },
        });
    })
    .forget();
}
