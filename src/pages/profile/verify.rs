use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use log::info;
use matrix_sdk::encryption::verification::{SasVerification, Verification};

use crate::{
    components::atoms::Button, hooks::use_client::use_client,
    utils::i18n_get_key_value::i18n_get_key_value,
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

#[inline_props]
pub fn Verify(cx: Scope, id: String) -> Element {
    let is_verified = use_ref::<bool>(cx, || false);

    let emoji = use_state::<Option<SasVerification>>(cx, || None);
    let client = use_client(cx).get();

    let task_wait_confirmation = use_coroutine(cx, |mut rx: UnboundedReceiver<SasVerification>| {
        to_owned![emoji];

        async move {
            while let Some(sas) = rx.next().await {
                emoji.set(Some(sas));
                info!("Confirm with `yes` or cancel with `no`: ");
            }
        }
    })
    .clone();

    use_coroutine(cx, |mut rx: UnboundedReceiver<bool>| {
        to_owned![task_wait_confirmation, client, is_verified];

        async move {
            client.add_event_handler(
                |ev: ToDeviceKeyVerificationRequestEvent, client: Client| async move {
                    info!("here ToDeviceKeyVerificationRequestEvent");
                    let request = client
                        .encryption()
                        .get_verification_request(&ev.sender, &ev.content.transaction_id)
                        .await
                        .expect("Request object wasn't created");

                    request
                        .accept()
                        .await
                        .expect("Can't accept verification request");
                },
            );

            client.add_event_handler(
                |ev: ToDeviceKeyVerificationStartEvent, client: Client| async move {
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
                        // print_devices(&ev.sender, &client).await;
                        sas.accept().await.unwrap();
                    }
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
                        // emoji.set(Some(sas));
                    }
                }
            });

            let x = is_verified.clone();
            client.add_event_handler(
                move |ev: ToDeviceKeyVerificationDoneEvent, client: Client| {
                    to_owned![x];

                    async move {
                        if let Some(Verification::SasV1(sas)) = client
                            .encryption()
                            .get_verification(&ev.sender, ev.content.transaction_id.as_str())
                            .await
                        {
                            if sas.is_done() {
                                x.set(true);
                                // print_devices(&ev.sender, &client).await;
                            }
                        }
                    }
                },
            );

            client.add_event_handler(
                |ev: OriginalSyncRoomMessageEvent, client: Client| async move {
                    info!("here OriginalSyncRoomMessageEvent");

                    if let MessageType::VerificationRequest(_) = &ev.content.msgtype {
                        let request = client
                            .encryption()
                            .get_verification_request(&ev.sender, &ev.event_id)
                            .await
                            .expect("Request object wasn't created");

                        request
                            .accept()
                            .await
                            .expect("Can't accept verification request");
                    }
                },
            );

            client.add_event_handler(
                    |ev: OriginalSyncKeyVerificationStartEvent, client: Client| async move {
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
                            // print_devices(&ev.sender, &client).await;
                            sas.accept().await.unwrap();
                        }
                    },
                );

            client.add_event_handler(
                    |ev: OriginalSyncKeyVerificationKeyEvent, client: Client| async move {
                        if let Some(Verification::SasV1(sas)) = client
                            .encryption()
                            .get_verification(&ev.sender, ev.content.relates_to.event_id.as_str())
                            .await
                        {
                            info!("here OriginalSyncKeyVerificationKeyEvent this function need task_wait_confirmation");
                            // task_wait_confirmation.send(sas);
                        }
                    },
                );

            client.add_event_handler(
                move |ev: OriginalSyncKeyVerificationDoneEvent, client: Client| {
                    to_owned![is_verified];

                    async move {
                        if let Some(Verification::SasV1(sas)) = client
                            .encryption()
                            .get_verification(&ev.sender, ev.content.relates_to.event_id.as_str())
                            .await
                        {
                            if sas.is_done() {
                                is_verified.set(true);
                                // print_devices(&ev.sender, &client).await;
                            }
                        }
                    }
                },
            );

            client.sync(SyncSettings::new()).await;
        }
        // }
    });

    let on_handle_confirm = move |sas: SasVerification| {
        to_owned![is_verified, emoji];

        cx.spawn({
            let sas = sas.clone();
            let client = client.clone();
            let is_verified = is_verified.clone();

            async move {
                sas.confirm().await.unwrap();

                if sas.is_done() {
                    is_verified.set(true);
                } else {
                    emoji.set(None);
                }
            }
        })
    };

    let on_handle_cancel = move |sas: SasVerification| {
        to_owned![emoji, is_verified];

        cx.spawn({
            let sas = sas.clone();

            async move {
                sas.cancel().await.unwrap();

                if sas.is_cancelled() {
                    is_verified.set(false);
                    emoji.set(None);
                }
            }
        })
    };

    render! {
        if !*is_verified.read() {
            rsx!(
                h2 {
                    style: r#"
                        margin-top: 40px;
                    "#,
                    "Verificar sesion"
                }

                div {
                    style: "
                        margin-top: 24px
                    ",

                    match emoji.get(){
                        Some(sas) => {
                            let emojis = sas.emoji().expect("emoji shoudl be available now");

                                rsx!(
                                    p {
                                        style: r#"
                                            margin-top: 12px;
                                        "#,
                                        "Verifica si los emojis coinciden con la otra sesion, en el mismo orden"
                                    }
                                    div {
                                        style: "
                                            display: grid;
                                            grid-template-columns: repeat(4, 25%);
                                            grid-template-rows: 80px 80px;
                                            gap: 8px;
                                            width: calc(100% - 24px)
                                        ",
                                        emojis.into_iter().map(|emoji| {
                                            rsx!(
                                                div {
                                                    style: "
                                                        display: flex;
                                                        flex-direction: column;
                                                        gap: 8px;
                                                        padding: 8px;
                                                        border-radius: 4px;
                                                        box-shadow: 0px 0px 4px 0px rgba(0,0,0,0.25);
                                                        align-items: center;
                                                    ",
                                                    span {
                                                        style: "
                                                            font-size: 30px;
                                                        ",
                                                        "{emoji.symbol}"
                                                    }
                                                    p {
                                                        style: "
                                                            font-size: 12px;
                                                        ",
                                                        "{emoji.description}"
                                                    }
                                                }
                                            )
                                        })
                                    }
                                    div {
                                        style: "
                                            margin-top: 24px;
                                        ",
                                        class: "row",
                                        Button {
                                            text: "No coincide",
                                            on_click: move |_| {
                                                on_handle_cancel(sas.clone());
                                            }
                                        }
                                        Button {
                                            text: "Si, coincide",
                                            on_click: move |_| {
                                                on_handle_confirm(sas.clone());
                                            }
                                        }
                                    }
                                )

                        }
                        None => {
                            rsx!(
                                div {
                                    "Para inicar la verificacion, ve a otro dispositivo desde el que iniciaste sesion y solicita la verificacion"
                                }
                            )
                        }

                    }
                }

            )
        }else {
            rsx!(
                h2 {
                    style: r#"
                        margin-top: 40px;
                    "#,
                    "Verificacion completada"
                }

                p {
                    style: r#"
                        margin-top: 12px;
                    "#,
                    "Haz verificado este dispositivo."
                }
            )
        }
    }
}
