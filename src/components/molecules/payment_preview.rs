use dioxus::prelude::{SvgAttributes, *};
use js_sys::Map;
use log::info;
use ruma::{serde::JsonObject, RoomId};
use std::ops::Deref;

use crate::{
    components::atoms::{button::Variant, Button, Card, File, InputMoney},
    hooks::{use_attach::use_attach, use_client::use_client, use_room::use_room},
    services::matrix::matrix::{send_message, send_payment, FileContent, PaymentEventContent},
};

use ruma::events::room::message::CustomEventContent;

use super::input_message::Payment;

pub fn PaymentPreview<'a>(cx: Scope<'a>) -> Element<'a> {
    let message_field = use_state(cx, String::new);
    let client = use_client(cx);
    let room = use_room(cx);
    let payment = use_shared_state::<Option<Payment>>(cx).unwrap();

    let attach_file_style = r#"
        height: 100%;
        width: 100%;
        object-fit: contain;
        border: 0.5px solid #0001;
        position: relative;
        background: var(--background-loud);
    "#;

    let attach_preview = r#"
        height: 100vh;
    "#;

    let on_handle_card = move |_| {
        to_owned![payment];

        *payment.write() = None;
    };

    let on_handle_pay = move |_: MouseEvent| {
        cx.spawn({
            to_owned![client, room, payment];

            async move {
                let current_room = room.get();
                // let custom = CustomEventContent {
                //     msgtype: String::from("m.fido.pay"),
                //     body: String::from("hola"),
                //     data: serde_json::from_str(
                //         "
                //         value: 1000000,
                //         asset: KSM,
                //         tx_id: random_tx_id,
                //     ",
                //     )
                //     .unwrap(),
                // };

                let data = r#"
                {
                    "name": "John Doe",
                    "age": 43,
                    "phones": [
                        "+44 1234567",
                        "+44 2345678"
                    ]
                }"#;

                let msg = ruma::events::room::message::MessageType::new(
                    "m.fido.pay",
                    String::from("hola"),
                    serde_json::from_str(data).unwrap(),
                );

                if let Ok(x) = msg {
                    // info!("message type payment preview {:#?}", x);

                    //     send_message(
                    //         &client.get(),
                    //         &RoomId::parse(current_room.id).unwrap(),
                    //         x,
                    //         None,
                    //         None,
                    //         None,
                    //     )
                    //     .await;

                    send_payment(
                        &client.get(),
                        &RoomId::parse(current_room.id).unwrap(),
                        PaymentEventContent {
                            asset: String::from("KSM"),
                            value: 10000,
                        },
                    )
                    .await;

                    *payment.write() = None;
                };
            }
        })
    };

    cx.render(rsx!(
        article {
            style: "
                    height: calc(100vh - 64px - 22px);
                    background: var(--background);
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    padding: 24px;
                    flex-direction: column;
                    position: fixed;
                    bottom: 0;
                    left: 0;
                    width: 100vw;
                ",
            div {
                style: "
                        background: var(--background-modal);
                        padding: 24px;
                        border-radius: 16px;
                    ",
                div {
                    style: "margin-top: 24px;",

                    // span {
                    //     style: "
                    //             color: var(--secondary-red-100, #DF1C41);
                    //             font-family: Inter;
                    //             font-size: 52px;
                    //             font-style: normal;
                    //             font-weight: 500;
                    //             line-height: 90%; /* 46.8px */
                    //             letter-spacing: -1.04px;
                    //         ",
                    //     "0.0320"
                    // }
                    InputMoney {
                        message: "{message_field}",
                        placeholder: "0",
                        on_input: move |_| {},
                        on_keypress: move |_| {},
                        on_click: move |_| {},
                        error: None
                    }

                    div {
                        class: "row",
                        style: "margin-top: 24px;",
                        div {
                            Button {
                                text: "Enviar",
                                variant: &Variant::Primary,
                                on_click: on_handle_pay
                            }
                        }

                        div {
                            Button {
                                text: "Cancelar",
                                variant: &Variant::Secondary,
                                on_click: on_handle_card
                            }
                        }

                    }
                }
            }
        }
    ))
}
