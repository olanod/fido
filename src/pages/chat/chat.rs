use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::molecules::modal::{ModalForm, RoomType};
use crate::components::molecules::Modal;
use crate::hooks::use_listen_message::use_listen_message;
use crate::hooks::use_modal::use_modal;
use crate::hooks::use_notification::use_notification;
use crate::pages::route::Route;

use crate::components::atoms::message::Messages;
use crate::components::atoms::Notification;
use crate::components::molecules::input_message::ReplyingTo;
use crate::components::molecules::rooms::CurrentRoom;
use crate::hooks::use_attach::AttachFile;
use crate::services::matrix::matrix::TimelineMessageEvent;

use log::info;
use matrix_sdk::room::Room;

#[derive(Debug, Clone)]
pub struct NotificationItem {
    pub title: String,
    pub body: String,
    pub show: bool,
}

pub struct MessageItem {
    pub room_id: String,
    pub msg: String,
    pub reply_to: Option<String>,
}

pub struct MessageEvent {
    pub room: Room,
    pub mgs: Option<TimelineMessageEvent>,
}

#[derive(Debug)]
pub struct ListHeight {
    pub height: String,
}

#[inline_props]
pub fn Chat(cx: Scope) -> Element {
    use_shared_state_provider::<CurrentRoom>(cx, || CurrentRoom {
        id: String::from(""),
        name: String::from(""),
        avatar_uri: None,
    });
    use_shared_state_provider::<Messages>(cx, || Vec::new());
    use_shared_state_provider::<Option<AttachFile>>(cx, || None);
    use_shared_state_provider::<Option<ReplyingTo>>(cx, || None);
    use_shared_state_provider::<ListHeight>(cx, || ListHeight {
        height: { format!("height: calc(100vh - 72px - {}px );", 82) },
    });

    use_shared_state_provider::<NotificationItem>(cx, || NotificationItem {
        title: String::from(""),
        body: String::from(""),
        show: false,
    });

    let notification = use_notification(cx);

    use_listen_message(cx);

    let _centered = r#"
        width:100%;
        display: flex;
        justify-content: center;
        align-items: center;
    "#;

    let modal = use_modal(cx);
    let navigator = use_navigator(cx);

    render! {
        if notification.get().show {
            rsx!(
                Notification {
                    title: "{notification.get().title}",
                    body: "{notification.get().body}",
                    on_click: move |_| info!("click notification")
                }
            )
        }
        if modal.get().show {
            rsx!(
                Modal {
                    on_click: move |event: ModalForm| {
                        match event.value {
                            RoomType::CHAT => {
                                modal.hide();
                                navigator.push(Route::RoomNew {});
                            },
                            RoomType::GROUP => {
                                modal.hide();
                                navigator.push(Route::RoomGroup {});
                            },
                            RoomType::CHANNEL => {
                                modal.hide()
                            },
                        }
                    },
                    on_close:move |_|{
                        modal.hide()
                    }
                }
            )
        }
        Outlet::<Route> {}
    }
}
