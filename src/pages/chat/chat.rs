use dioxus::prelude::*;
use dioxus_router::prelude::*;
use matrix_sdk::encryption::verification::SasVerification;

use crate::components::molecules::modal::{ModalForm, RoomType};
use crate::components::molecules::Modal;
use crate::hooks::use_listen_message::use_listen_message;
use crate::hooks::use_modal::use_modal;
use crate::hooks::use_notification::use_notification;
use crate::pages::route::Route;

use crate::components::atoms::Notification;
use crate::services::matrix::matrix::TimelineMessageEvent;

use matrix_sdk::room::Room;

#[derive(Debug, Clone)]
pub struct NotificationItem {
    pub title: String,
    pub body: String,
    pub show: bool,
    pub handle: NotificationHandle,
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

// #[derive(Debug, Clone)]
// pub struct NotificationAction<T> {
//     pub redirect: Option<String>,
//     pub meta: T,
// }

#[inline_props]
pub fn Chat(cx: Scope) -> Element {
    let notification = use_notification(cx);
    let modal = use_modal(cx);
    let navigator = use_navigator(cx);

    use_listen_message(cx);

    let _centered = r#"
        width:100%;
        display: flex;
        justify-content: center;
        align-items: center;
    "#;

    render! {
        if notification.get().show {
            rsx!(
                Notification {
                    title: "{notification.get().title}",
                    body: "{notification.get().body}",
                    on_click: move |_| {
                       match notification.get().handle.value {
                            NotificationType::Click => {
                                // if let Some(route) = redirect {

                                // }
                            },
                            NotificationType::AcceptSas(sas, redirect) => {
                                cx.spawn({
                                    let navigator = navigator.clone();

                                    async move {
                                        let x = sas.accept().await;

                                        // match x {
                                        //     Ok(info) => {
                                        //         info!("information about accept sas: {:?}", info);
                                        //         if let Some(route) = redirect {
                                        //             navigator.push(route);
                                        //         }
                                        //     },
                                        //     Err(err) => {
                                        //         info!("{err}")
                                        //     }
                                        // }

                                    }
                                });
                            }
                            NotificationType::None => {

                            }
                       }
                    }
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
