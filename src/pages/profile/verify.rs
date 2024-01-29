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
    let i18 = use_i18(cx);

    let key_verify_unverified_cta_match = translate!(i18, "verify.unverified.cta_match");
    let key_verify_unverified_cta_disagree = translate!(i18, "verify.unverified.cta_disagree");

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
<<<<<<< HEAD
        div {
            class: "page--clamp",
            if !*is_verified.read() {
                rsx!(
                    h2 {
                        class: "verify__title",
                        "Verificar sesion"
                    }
=======
        if !*is_verified.read() {
            rsx!(
                h2 {
                    class: "verify__title",
                    translate!(i18, "verify.unverified.title")
                }
>>>>>>> 190ae6f (ref(i18n): complete translations)

                    div {
                        class: "verify__spacer",
                        match emoji.get(){
                            Some(sas) => {
                                let emojis = sas.emoji().expect("emoji shoudl be available now");

<<<<<<< HEAD
                                    rsx!(
                                        p {
                                            class: "verify__description",
                                            "Verifica si los emojis coinciden con la otra sesion, en el mismo orden"
                                        }
                                        div {
                                            class: "verify__wrapper",
                                            emojis.into_iter().map(|emoji| {
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
=======
                                rsx!(
                                    p {
                                        class: "verify__description",
                                        translate!(i18, "verify.unverified.question")
                                    }
                                    div {
                                        class: "verify__wrapper",
                                        emojis.into_iter().map(|emoji| {
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
>>>>>>> 190ae6f (ref(i18n): complete translations)
                                                    }
                                                )
                                            })
                                        }
                                        div {
                                            class: "verify__spacer row",
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
<<<<<<< HEAD
                                        class: "verify__info",
                                        "Para inicar la verificacion, ve a otro dispositivo desde el que iniciaste sesion y solicita la verificacion"
=======
                                        class: "verify__spacer row",
                                        Button {
                                            text: "{key_verify_unverified_cta_disagree}",
                                            on_click: move |_| {
                                                on_handle_cancel(sas.clone());
                                            }
                                        }
                                        Button {
                                            text: "{key_verify_unverified_cta_match}",
                                            on_click: move |_| {
                                                on_handle_confirm(sas.clone());
                                            }
                                        }
>>>>>>> 190ae6f (ref(i18n): complete translations)
                                    }
                                )
                            }

                        }
<<<<<<< HEAD
=======
                        None => {
                            rsx!(
                                div {
                                    class: "verify__info",
                                    translate!(i18, "verify.unverified.description")
                                }
                            )
                        }

>>>>>>> 190ae6f (ref(i18n): complete translations)
                    }

<<<<<<< HEAD
                )
            } else {
                rsx!(
                    h2 {
                        class: "verify__title--verified",
                        "Verificacion completada"
                    }

                    p {
                        class: "verify__description--verified",
                        "Haz verificado este dispositivo."
                    }
                )
            }
=======
            )
        } else {
            rsx!(
                h2 {
                    class: "verify__title--verified",
                    translate!(i18, "verify.verified.title")
                }

                p {
                    class: "verify__description--verified",
                    translate!(i18, "verify.verified.description")
                }
            )
>>>>>>> 190ae6f (ref(i18n): complete translations)
        }
    }
}
