use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
use log::info;
use matrix_sdk::encryption::verification::{format_emojis, Emoji, SasVerification};
use std::{collections::HashMap, ops::Deref};

use crate::{
    components::atoms::Button,
    hooks::{
        use_attach::{use_attach, AttachFile},
        use_client::use_client,
    },
    utils::i18n_get_key_value::i18n_get_key_value,
};

#[inline_props]
pub fn Verify(cx: Scope, id: String) -> Element {
    let sas =
        use_shared_state::<Option<SasVerification>>(cx).expect("Emoji not provided in verify");
    let client = use_client(cx);
    let is_verified = use_ref::<bool>(cx, || false);

    let on_handle_confirm = move |sas: SasVerification| {
        cx.spawn({
            let sas = sas.clone();
            let client = client.clone();
            let is_verified = is_verified.clone();

            async move {
                let x = sas.confirm().await.unwrap();

                info!("is_cancelled: {}", sas.is_cancelled());
                info!("is_done: {}", sas.is_done());

                // let client = client.get();

                // let user_id = client.user_id();
                // let device_id = client.session().unwrap().device_id;

                // if let Ok(result) = client
                //     .encryption()
                //     .get_device(user_id.unwrap(), &device_id)
                //     .await
                // {
                //     if let Some(device) = result {
                //         is_verified.set(device.is_verified());

                //         info!("{:?}", device.is_verified());
                //     }
                // }
            }
        })
    };

    let on_handle_cancel = move |sas: SasVerification| {
        cx.spawn({
            let sas = sas.clone();

            async move {
                sas.cancel().await.unwrap();
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

                p {
                    style: r#"
                        margin-top: 12px;
                    "#,
                    "Verifica si los emojis coinciden con la otra sesion, en el mismo orden"
                }
                div {
                    style: "
                        margin-top: 24px
                    ",

                    if let Some(sas) = sas.read().clone() {
                        let y = sas.clone();
                        let emojis = sas.clone().emoji().expect("emoji shoudl be available now");

                        rsx!(
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
                                // Button {
                                //     text: "No coincide",
                                //     on_click: move |_| {
                                //         on_handle_cancel(y.clone());
                                //     }
                                // }
                                Button {
                                    text: "Si, coincide",
                                    on_click: move |_| {
                                        on_handle_confirm(sas.clone());
                                    }
                                }
                            }
                        )
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
