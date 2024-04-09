use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use log::info;
use matrix_sdk::encryption::verification::{SasVerification, Verification};

use crate::{
    components::atoms::Button,
    hooks::{use_client::use_client, use_notification::use_notification},
};

use futures_util::StreamExt;

use matrix_sdk::{
    self,
    config::SyncSettings,
    ruma::events::{
        key::verification::{
            done::{OriginalSyncKeyVerificationDoneEvent, ToDeviceKeyVerificationDoneEvent},
            key::{OriginalSyncKeyVerificationKeyEvent, ToDeviceKeyVerificationKeyEvent},
            request::ToDeviceKeyVerificationRequestEvent,
            start::{OriginalSyncKeyVerificationStartEvent, ToDeviceKeyVerificationStartEvent},
        },
        room::message::{MessageType, OriginalSyncRoomMessageEvent},
    },
    Client,
};

pub enum VerificationError {
    FlowNotFound,
    SasAcceptFailed,
    SasConfirmFailed,
    SasCancelFailed,
    SyncFailed,
}

#[component]
pub fn Verify(id: String) -> Element {
    let _ = &id;
    let i18 = use_i18();
    let client = use_client().get();
    let mut notification = use_notification();

    let mut is_verified = use_signal::<bool>(|| false);
    let mut emoji = use_signal::<Option<SasVerification>>(|| None);
    let mut sas = use_signal::<Option<SasVerification>>(|| None);

    let task_wait_confirmation =
        use_coroutine(|mut rx: UnboundedReceiver<SasVerification>| async move {
            while let Some(sas) = rx.next().await {
                emoji.set(Some(sas));
                info!("Confirm with `yes` or cancel with `no`: ");
            }
        })
        .clone();

    let task_verify = use_coroutine(|mut rx: UnboundedReceiver<bool>| async move {
        while let Some(verify) = rx.next().await {
            is_verified.set(verify);
        }
    });

    let task_handle_error =
        use_coroutine(|mut rx: UnboundedReceiver<VerificationError>| async move {
            while let Some(e) = rx.next().await {
                let message = match e {
                    VerificationError::FlowNotFound => {
                        translate!(i18, "verify.errors.flow_not_found")
                    }
                    VerificationError::SasAcceptFailed => {
                        translate!(i18, "verify.errors.sas_accept")
                    }
                    VerificationError::SasConfirmFailed => {
                        translate!(i18, "verify.errors.sas_confirm")
                    }
                    VerificationError::SasCancelFailed => {
                        translate!(i18, "verify.errors.sas_cancel")
                    }
                    VerificationError::SyncFailed => translate!(i18, "verify.errors.sas_sync"),
                };
                notification.handle_error(&message);
            }
        });

    use_coroutine(|mut _rx: UnboundedReceiver<()>| async move {
        client.add_event_handler(
            move |ev: ToDeviceKeyVerificationRequestEvent, client: Client| async move {
                info!("here ToDeviceKeyVerificationRequestEvent");
                let request = client
                    .encryption()
                    .get_verification_request(&ev.sender, &ev.content.transaction_id)
                    .await
                    .expect("Request object wasn't created");

                if let Err(_) = request.accept().await {
                    task_handle_error.send(VerificationError::SasAcceptFailed)
                };
            },
        );

        client.add_event_handler(
            move |ev: ToDeviceKeyVerificationStartEvent, client: Client| async move {
                if let Some(Verification::SasV1(sas)) = client
                    .encryption()
                    .get_verification(&ev.sender, ev.content.transaction_id.as_str())
                    .await
                {
                    info!(
                        "ToDeviceKeyVerificationStartEvent Starting verification with {} {}",
                        &sas.other_device().user_id(),
                        &sas.other_device().device_id()
                    );
                    if let Err(_) = sas.accept().await {
                        task_handle_error.send(VerificationError::SasAcceptFailed);
                    };
                }
            },
        );

        client.add_event_handler(
            move |ev: ToDeviceKeyVerificationKeyEvent, client: Client| async move {
                if let Some(Verification::SasV1(sas)) = client
                    .encryption()
                    .get_verification(&ev.sender, ev.content.transaction_id.as_str())
                    .await
                {
                    task_wait_confirmation.send(sas);
                }
            },
        );

        client.add_event_handler(
            move |ev: ToDeviceKeyVerificationDoneEvent, client: Client| async move {
                if let Some(Verification::SasV1(sas)) = client
                    .encryption()
                    .get_verification(&ev.sender, ev.content.transaction_id.as_str())
                    .await
                {
                    if sas.is_done() {
                        task_verify.send(true);
                    }
                }
            },
        );

        client.add_event_handler(
            move |ev: OriginalSyncKeyVerificationStartEvent, client: Client| async move {
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
                    if let Err(_) = sas.accept().await {
                        task_handle_error.send(VerificationError::SasAcceptFailed)
                    };
                }
            },
        );

        client.add_event_handler(
            move |ev: OriginalSyncRoomMessageEvent, client: Client| async move {
                info!("here OriginalSyncRoomMessageEvent");

                if let MessageType::VerificationRequest(_) = &ev.content.msgtype {
                    let request = client
                        .encryption()
                        .get_verification_request(&ev.sender, &ev.event_id)
                        .await
                        .expect("Request object wasn't created");

                    if let Err(_) = request.accept().await {
                        task_handle_error.send(VerificationError::SasAcceptFailed);
                    };
                }
            },
        );

        client.add_event_handler(
                    |ev: OriginalSyncKeyVerificationKeyEvent, client: Client| async move {
                        if let Some(Verification::SasV1(_)) = client
                            .encryption()
                            .get_verification(&ev.sender, ev.content.relates_to.event_id.as_str())
                            .await
                        {
                            info!("here OriginalSyncKeyVerificationKeyEvent this function need task_wait_confirmation");
                        }
                    },
                );

        client.add_event_handler(
            move |ev: OriginalSyncKeyVerificationDoneEvent, client: Client| async move {
                if let Some(Verification::SasV1(sas)) = client
                    .encryption()
                    .get_verification(&ev.sender, ev.content.relates_to.event_id.as_str())
                    .await
                {
                    if sas.is_done() {
                        task_verify.send(true);
                    }
                }
            },
        );

        if let Err(_) = client.sync(SyncSettings::new()).await {
            task_handle_error.send(VerificationError::SyncFailed)
        };
    });

    let on_handle_confirm = move |_| {
        spawn({
            async move {
                let Some(sas) = sas() else {
                    return;
                };

                if let Err(_) = sas.confirm().await {
                    task_handle_error.send(VerificationError::SasConfirmFailed)
                };

                if sas.is_done() {
                    is_verified.set(true);
                } else {
                    emoji.set(None);
                }
            }
        });
    };

    let on_handle_cancel = move |_| {
        spawn({
            async move {
                let Some(sas) = sas() else {
                    return;
                };

                if let Err(_) = sas.cancel().await {
                    task_handle_error.send(VerificationError::SasCancelFailed)
                };

                if sas.is_cancelled() {
                    is_verified.set(false);
                    emoji.set(None);
                }
            }
        });
    };

    rsx! {
        if !*is_verified.read() {
            h2 { class: "verify__title", {translate!(i18, "verify.unverified.title")} }

            div { class: "verify__spacer",
                match emoji(){
                    Some(s) => {
                        let emojis = s.emoji().expect("emoji shoudl be available now");
                        sas.set(Some(s));
                
                            rsx!(
                                p {
                                    class: "verify__description",
                                    {translate!(i18, "verify.unverified.question")}
                                }
                                div {
                                    class: "verify__wrapper",
                                    {emojis.into_iter().map(|emoji| {
                                        rsx!(
                                            div {
                                                class: "verify__emojis",
                                                span {
                                                    class: "verify__method__title",
                                                    "{emoji.symbol}"
                                                }
                                                p {
                                                    class: "verify__method__description",
                                                    "{emoji.description}"
                                                }
                                            }
                                        )
                                    })}
                                }
                                div {
                                    class: "verify__spacer row",
                                    Button {
                                        text: translate!(i18, "verify.unverified.cta_disagree"),
                                        status: None,
                                        on_click: on_handle_cancel
                                    }
                                    Button {
                                        text: translate!(i18, "verify.unverified.cta_match"),
                                        status: None,
                                        on_click: on_handle_confirm
                                    }
                                }
                            )
                
                    }
                    None => {
                        rsx!(
                            div {
                                class: "verify__info",
                                {translate!(i18, "verify.unverified.description")}
                            }
                        )
                    }
                
                }
            }
        } else {

            h2 { class: "verify__title--verified", {translate!(i18, "verify.verified.title")} }

            p { class: "verify__description--verified",
                {translate!(i18, "verify.verified.description")}
            }
        }
    }
}
