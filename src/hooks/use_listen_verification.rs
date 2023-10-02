use dioxus::prelude::*;
use futures_util::StreamExt;
use log::info;
use matrix_sdk::{
    config::SyncSettings,
    encryption::verification::{
        format_emojis, Emoji, SasVerification, Verification, VerificationRequest,
    },
    ruma::{
        events::{
            key::verification::{
                done::{OriginalSyncKeyVerificationDoneEvent, ToDeviceKeyVerificationDoneEvent},
                key::{OriginalSyncKeyVerificationKeyEvent, ToDeviceKeyVerificationKeyEvent},
                request::ToDeviceKeyVerificationRequestEvent,
                start::{OriginalSyncKeyVerificationStartEvent, ToDeviceKeyVerificationStartEvent},
            },
            room::message::{MessageType, OriginalSyncRoomMessageEvent},
        },
        UserId,
    },
    Client,
};

use crate::{
    components::{
        atoms::notification, molecules::rooms::CurrentRoom,
        organisms::chat::utils::handle_notification,
    },
    pages::{
        chat::chat::{MessageEvent, NotificationHandle, NotificationItem, NotificationType},
        route::Route,
    },
};

use super::{use_client::use_client, use_notification::use_notification};

#[allow(clippy::needless_return)]
pub fn use_listen_verification(cx: &ScopeState) -> &UseListenMessagesState {
    let client = use_client(cx).get();
    let handler_added = use_ref(cx, || false);
    let notification = use_notification(cx);
    let emoji = use_shared_state::<Option<SasVerification>>(cx).expect("Emoji not provided");

    let task_handle_notification =
        use_coroutine(cx, |mut rx: UnboundedReceiver<NotificationItem>| {
            to_owned![notification];

            async move {
                while let Some(n) = rx.next().await {
                    handle_notification(n, notification.to_owned());
                }
            }
        })
        .clone();

    let task_wait_confirmation = use_coroutine(cx, |mut rx: UnboundedReceiver<SasVerification>| {
        to_owned![emoji];

        async move {
            while let Some(sas) = rx.next().await {
                *emoji.write() = Some(sas);
                info!("Confirm with `yes` or cancel with `no`: ");
            }
        }
    })
    .clone();
    let c = client.clone();

    use_coroutine(cx, |_: UnboundedReceiver<String>| {
        to_owned![
            client,
            handler_added,
            task_wait_confirmation,
            notification // task_print_devices
        ];

        info!("loading verification");

        async move {
            client.add_event_handler(
                |ev: ToDeviceKeyVerificationRequestEvent, client: Client| async move {
                    let request = client
                        .encryption()
                        .get_verification_request(&ev.sender, &ev.content.transaction_id)
                        .await
                        .expect("Request object wasn't created");

                    info!("here ToDeviceKeyVerificationRequestEvent");
                    request
                        .accept()
                        .await
                        .expect("Can't accept verification request");
                },
            );

            client.add_event_handler(move |ev: ToDeviceKeyVerificationKeyEvent, client: Client| {
                // let task_wait_confirmation = task_wait_confirmation.clone();

                to_owned![task_wait_confirmation];

                async move {
                    if let Some(Verification::SasV1(sas)) = client
                        .encryption()
                        .get_verification(&ev.sender, ev.content.transaction_id.as_str())
                        .await
                    {
                        task_wait_confirmation.send(sas);
                    }
                }
            });

            client.add_event_handler(
                |ev: ToDeviceKeyVerificationDoneEvent, client: Client| async move {
                    if let Some(Verification::SasV1(sas)) = client
                        .encryption()
                        .get_verification(&ev.sender, ev.content.transaction_id.as_str())
                        .await
                    {
                        if sas.is_done() {
                            info!("sas is done")
                            // print_result(&sas);
                            // task_print_devices.send(&ev.sender).await;
                        }
                    }
                },
            );

            client.add_event_handler(
                |ev: OriginalSyncRoomMessageEvent, client: Client| async move {
                    if let MessageType::VerificationRequest(_) = &ev.content.msgtype {
                        let request = client
                            .encryption()
                            .get_verification_request(&ev.sender, &ev.event_id)
                            .await
                            .expect("Request object wasn't created");

                        info!("here OriginalSyncRoomMessageEvent");
                        request
                            .accept()
                            .await
                            .expect("Can't accept verification request");
                    }
                },
            );

            client.add_event_handler(
                move |ev: OriginalSyncKeyVerificationStartEvent, client: Client| {
                    // to_owned![notification];

                    async move {
                        if let Some(Verification::SasV1(sas)) = client
                            .encryption()
                            .get_verification(&ev.sender, ev.content.relates_to.event_id.as_str())
                            .await
                        {
                            info!(
                                "OriginalSyncKeyVerificationStartEvent Starting verification with {} {}",
                                &sas.other_device().user_id(),
                                &sas.other_device().device_id()
                            );

                            // handle_notification(
                            //     NotificationItem {
                            //         title: String::from("Solicitud de verificacion"),
                            //         body: String::from("Da click para verificar esta sesion"),
                            //         show: true,
                            //     },
                            //     notification.to_owned(),
                            // );
                            // print_devices(&ev.sender, &client).await;
                            // sas.accept().await.unwrap();
                        }
                    }
                },
            );
        }
    });

    use_coroutine(cx, |_: UnboundedReceiver<String>| {
        to_owned![task_handle_notification];

        async move {
            client.add_event_handler(
                move |ev: ToDeviceKeyVerificationStartEvent, client: Client| {
                    // to_owned![sas];
                    let task_handle_notification = task_handle_notification.clone();
                    // let sas = sas.clone();

                    async move {
                        if let Some(Verification::SasV1(sas_verification)) = client
                            .encryption()
                            .get_verification(&ev.sender, ev.content.transaction_id.as_str())
                            .await
                        {
                            info!(
                                "ToDeviceKeyVerificationStartEvent Starting verification with {} {}",
                                &sas_verification.other_device().user_id(),
                                &sas_verification.other_device().device_id()
                            );

                            task_handle_notification.send(NotificationItem {
                                title: String::from("Solicitud de verificacion"),
                                body: String::from("Da click para verificar esta sesion"),
                                show: true,
                                handle: NotificationHandle {
                                    value: NotificationType::AcceptSas(
                                        sas_verification,
                                        Some(Route::Verify {
                                            id: String::from("fidoid"),
                                        }),
                                    ),
                                },
                            })

                            // task_print_devices.send(&ev.sender).await;
                            // sas.accept().await.unwrap();
                        }
                    }
                },
            );
        }
    });
    let client = c.clone();
    use_coroutine(cx, |_: UnboundedReceiver<String>| {
        to_owned![client, task_wait_confirmation];

        async move {
            client.add_event_handler(
                move |ev: OriginalSyncKeyVerificationKeyEvent, client: Client| {
                    // let task_wait_confirmation = task_wait_confirmation.clone();
                    to_owned![task_wait_confirmation];

                    async move {
                        if let Some(Verification::SasV1(sas)) = client
                            .encryption()
                            .get_verification(&ev.sender, ev.content.relates_to.event_id.as_str())
                            .await
                        {
                            task_wait_confirmation.send(sas);
                        }
                    }
                },
            );

            client.add_event_handler(
                |ev: OriginalSyncKeyVerificationDoneEvent, client: Client| async move {
                    if let Some(Verification::SasV1(sas)) = client
                        .encryption()
                        .get_verification(&ev.sender, ev.content.relates_to.event_id.as_str())
                        .await
                    {
                        if sas.is_done() {
                            // print_result(&sas);
                            // task_print_devices.send(&ev.sender).await;
                        }
                    }
                },
            );
        }
    });

    cx.use_hook(move || UseListenMessagesState {
        inner: String::from(""),
    })
}

#[derive(Clone)]
pub struct UseListenMessagesState {
    inner: String,
}

impl UseListenMessagesState {
    pub fn initialize(&self) {}
}
