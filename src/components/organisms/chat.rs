use std::collections::HashMap;
use std::ops::Deref;

use crate::components::atoms::header::{HeaderEvent, HeaderCallOptions};
use crate::components::atoms::helper::{Helper, HelperData};
use crate::components::atoms::message::{self, Message, Messages};
use crate::components::atoms::room::RoomItem;
use crate::components::atoms::{MessageReply, Header, Notification, RoomView, Spinner};
use crate::components::molecules::input_message::{FormMessageEvent, ReplyingTo};
use crate::components::molecules::rooms::{CurrentRoom, FormRoomEvent};
use crate::components::molecules::{InputMessage, RoomsList};
use crate::services::matrix::matrix::{
    join_room, list_rooms, room_member, send_message, timeline, TimelineMessageType, send_attachment, Attachment, TimelineMessageEvent, format_original_any_room_message_event, format_reply_from_event,
};
use crate::MatrixClientState;
use dioxus::prelude::*;
use futures_util::StreamExt;
use gloo::storage::LocalStorage;
use log::info;
use matrix_sdk::config::SyncSettings;
use matrix_sdk::room::Room;
use matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent;
use matrix_sdk::ruma::{RoomId, EventId};
use matrix_sdk::Client;
use message::MessageView;
use wasm_bindgen::JsCast;

use super::login::LoggedIn;

#[derive(Debug)]
pub struct NotificationItem {
    title: String,
    body: String,
    show: bool,
}

pub struct MessageItem {
    room_id: String,
    msg: String,
    reply_to: Option<String>
}

pub struct MessageAttachItem {
    msg: Attachment,
}

pub struct MessageEvent {
    room: Room,
    mgs: Option<TimelineMessageEvent>,
}

pub fn IndexChat(cx: Scope) -> Element {
    let matrix_client = use_shared_state::<MatrixClientState>(cx).unwrap();
    let logged_in = use_shared_state::<LoggedIn>(cx).unwrap();

    let messages = use_ref::<Messages>(cx, || Vec::new());
    let handler_added = use_ref(cx, || false);
    let next_id = use_ref(cx, || 0);
    let limit_events_by_room = use_ref::<HashMap<String, u64>>(cx, || HashMap::new());

    let messages_loading = use_state::<bool>(cx, || false);
    let message_field = use_state(cx, String::new);
    let notification = use_state::<NotificationItem>(cx, || NotificationItem {
        title: String::from(""),
        body: String::from(""),
        show: false,
    });

    let image_x = use_state::<Option<Vec<u8>>>(cx, || None);

    let client = matrix_client.read().client.clone().unwrap();

    let rooms_vec = list_rooms(&client);
    let rooms = use_state::<Vec<RoomItem>>(cx, || rooms_vec);
    let current_room = use_ref::<CurrentRoom>(cx, || CurrentRoom {
        id: String::new(),
        name: String::new(),
    });
    let replying_to = use_state::<Option<ReplyingTo>>(cx, || None);
    let height = use_state(cx, || format!("height: calc(100% - 50px - {}px );", 88));

    let task_timeline = use_coroutine(cx, |mut rx: UnboundedReceiver<bool>| {
        to_owned![
            client,
            messages,
            next_id,
            messages_loading,
            current_room,
            limit_events_by_room
        ];

        async move {
            while let Some(load_more) = rx.next().await {
                messages_loading.set(true);
                messages.set(Vec::new());

                let current_room_id = current_room.read().id.clone();
                let current_events = limit_events_by_room
                    .read()
                    .get(&current_room_id)
                    .unwrap_or_else(|| &(15 as u64))
                    .to_owned();

                if load_more {
                    limit_events_by_room
                        .with_mut(|lr| lr.insert(current_room_id, current_events + 15));
                } else {
                    limit_events_by_room.with_mut(|lr| lr.insert(current_room_id, current_events));
                }

                let room_id = RoomId::parse(current_room.read().id.clone()).unwrap();

                let msg = timeline(&client, &room_id, current_events).await;

                for m in msg.iter() {
                    let mut rep: Option<MessageReply> = None;

                    if let Some(r) = &m.reply {
                        rep = Some(MessageReply {
                            display_name: r.sender.name.clone(),
                            avatar_uri: r.sender.avatar_uri.clone(),
                            content: r.body.clone() ,
                        })
                    }

                    messages.with_mut(|messages| {
                        messages.push(Message {
                            id: *next_id.read(),
                            event_id: m.event_id.clone(),
                            display_name: m.sender.name.clone(),
                            content: m.body.clone(),
                            avatar_uri: m.sender.avatar_uri.clone(),
                            reply: rep.clone(),
                        });
                    });

                    let current_id = *next_id.read();
                    next_id.set(current_id + 1);
                }

                messages_loading.set(false);
            }
        }
    });

    let task_sender = use_coroutine(cx, |mut rx: UnboundedReceiver<MessageEvent>| {
        to_owned![messages, notification, next_id, current_room];

        async move {
            while let Some(message_event) = rx.next().await {
                if let Some(message) = message_event.mgs {
                    let mut reply = None;
                    
                    if let Some(r) = message.reply {
                        reply = Some(MessageReply { content: r.body, display_name: r.sender.name, avatar_uri: r.sender.avatar_uri });
                    }

                    let plain_message = match &message.body {
                        TimelineMessageType::Image(_) => "Imagen",
                        TimelineMessageType::Text(t) => t,
                    };

                    handle_notification(
                        NotificationItem {
                            title: String::from(message_event.room.name().unwrap()),
                            body: String::from(plain_message),
                            show: true,
                        },
                        notification.to_owned(),
                    );

                    let is_in_current_room = message_event.room.room_id().as_str().eq(&current_room.read().id);

                    if is_in_current_room {
                        messages.with_mut(|messages| {
                            messages.push(Message {
                                id: *next_id.read(),
                                event_id: message.event_id,
                                display_name: message.sender.name.clone(),
                                content: message.body.clone(),
                                avatar_uri: message.sender.avatar_uri.clone(),
                                reply: reply,
                            });

                            messages.rotate_right(1);

                            let current_id = *next_id.read();
                            next_id.set(current_id + 1);
                        });
                    }
                }
                
            }
        }
    })
    .clone();

    let task_push_message = use_coroutine(cx, |mut rx: UnboundedReceiver<MessageItem>| {
        to_owned![message_field, client, replying_to];

        async move {
            while let Some(message_item) = rx.next().await {
                if message_item.msg.starts_with('!') {
                    handle_command(message_item, &client).await;
                } else {
                    let room_id = RoomId::parse(message_item.room_id).unwrap();
                    let event_id = if let Some(e) = message_item.reply_to{
                         Some(EventId::parse(e).unwrap())
                    }else {
                        None
                    };
                    send_message(&client, &room_id, message_item.msg, event_id).await
                }

                message_field.set(String::new());
                replying_to.set(None);
            }
        }
    });

    let task_push_attach = use_coroutine(cx, |mut rx: UnboundedReceiver<Vec<u8>>| {
        to_owned![client, current_room];

        async move {
            while let Some(message_item) = rx.next().await {
                let room_id = RoomId::parse(current_room.read().id.clone()).unwrap();
                send_attachment(&client, &room_id, &Attachment {
                    data: message_item
                }).await;
            }
        }
    });

    // After logging is mandatory to perform a client sync,
    // since the chat needs sync to listen for new messages
    // this coroutine is necesary
    use_coroutine(cx, |_: UnboundedReceiver<String>| {
        to_owned![client, handler_added, task_sender];

        async move {
            if !*handler_added.read() {
                client.add_event_handler(move |ev: OriginalSyncRoomMessageEvent, room: Room| {
                    let t = task_sender.clone();
                    
                    async move {
                        let message_type = &ev.content.msgtype;
                        let event_id = ev.event_id;
                        let member = room_member(ev.sender, &room).await;
                        let relates = &ev.content.relates_to;
                        let message_result = format_original_any_room_message_event(&message_type, event_id, &member).await;
                        let message_result =
                        format_reply_from_event(&message_type, relates, &room, message_result, &member).await;
                        
                        t.send(MessageEvent {
                            room,
                            mgs: message_result,
                        })
                    }
                    
                });

                handler_added.set(true);
            }

            let _ = client.sync(SyncSettings::default()).await;
        }
    });

    let on_push_message = move |evt: FormMessageEvent| {
        let mut e = None;        
        
        if let Some(x) = replying_to.get(){
            e = Some(x.event_id.clone());
        }

        task_push_message.send(MessageItem {
            room_id: current_room.read().id.clone(),
            msg: evt.value,
            reply_to: e
        });
    };

    let on_click_room = move |evt: FormRoomEvent| {
        current_room.set(evt.room);
        task_timeline.send(false)
    };

    let log_out = move || {
        cx.spawn({
            to_owned![client, logged_in];

            async move {
                let _ = client.logout().await;
                let _ = <LocalStorage as gloo::storage::Storage>::delete("session_file");
                logged_in.write().is_logged_in = false;
            }
        });
    };

    let centered = r#"
        width:100%;
        display: flex;
        justify-content: center;
        align-items: center;
    "#;

    let df_value = use_state(cx, || "");

    let x = move |_| {
        cx.spawn({
            let df_value = df_value.to_owned();
            
            async move {
                df_value.set("!join !vaoCGcunXVlxJqWyjQ:matrix.org");
                df_value.needs_update();
            }
        })
    };

    let header_event = move |evt: HeaderEvent| {
        to_owned![current_room];

        match evt.value {
            HeaderCallOptions::CLOSE => {
                current_room.set(CurrentRoom {
                    id: String::new(),
                    name: String::new(),
                })
            }
        }
    };

    let input_message_event = move |evt: HeaderEvent| {
        to_owned![replying_to, height];

        match evt.value {
            HeaderCallOptions::CLOSE => {
                replying_to.set(None);
                height.set("height: calc(100% - 50px - 88px );".to_string());
            }
        }
    };

    cx.render(rsx! {
        if notification.show {
            rsx!(
                Notification {
                    title: notification.title.as_str(),
                    body: notification.body.as_str(),
                    on_click: move |_| info!("click notification")
                }
            )
        }

        section {
            class: "options",

                RoomsList {
                    rooms: rooms,
                    on_submit: on_click_room
                }

                div {
                    style: r#"
                        display: flex;
                        flex-direction:column;
                    "#,
                    RoomView {
                        room_avatar_uri: None,
                        room_id: "1",
                        room_name: "Cerrar sesion" ,
                        on_click: move |_| {
                            log_out()
                        }
                    }
                }

        }

        if current_room.read().id.len() > 0 {
            let loadmore_style = r#"
                width: fit-content;
                padding: 4px 20px;
                border-radius: 20px;
                border: 1px solid transparent;
                color: var(--text-1);
                background: var(--surface-0);
                box-shadow: 0px 2px 8px 0 rgba(118,131,156,.6);
                transition: opacity 0.2s ease-out, background-color 0.2s ease-out;
                cursor: pointer;
                margin: 0 auto;
            "#;

            rsx!{
                div {
                    style: "{height.get()}",
                    class:"messages-list",
                    // List {},
                    Header {
                        text: "{current_room.read().name.clone()}",
                        on_event: header_event
                    }
                    if !*messages_loading.get() {
                        rsx!(
                            messages.read().iter().map(|message| {
                                let message = message.clone();
                                let event = message.event_id.clone();
                                info!("{message:?}");
                                cx.render(rsx!(
                                    div {
                                        onclick: move |_| {
                                            let height = height.clone();
                                            if let Some(eid) = &event {
                                                let x = ReplyingTo { event_id: eid.clone(), content: message.content.clone(), display_name: message.display_name.clone(), avatar_uri: message.avatar_uri.clone() };
                                                replying_to.set(Some(x));
                                                
                                                let window = web_sys::window().expect("global window does not exists");
                                                let document = window.document().expect("expecting a document on window");
                                                let val = document.get_element_by_id("input_field").unwrap().dyn_into::<web_sys::HtmlElement>().unwrap(); 
                                                
                                                gloo::timers::callback::Timeout::new(50, move || {  
                                                    let h = val.offset_height();
                                                    let x = format!("height: calc(100% - 50px - {}px );", h); 
                                                    height.set(x);
                                                })
                                                .forget();
                                            }
                                        },
                                        MessageView {
                                            key: "{message.id}",
                                            message: Message {
                                                id: message.id,
                                                event_id: message.event_id,
                                                display_name: message.display_name.clone(),
                                                avatar_uri: message.avatar_uri.clone(),
                                                content: message.content.clone(),
                                                reply: message.reply.clone()
                                            },
                                            is_replying: false,
                                            on_event: move |_| {}
                                        }
                                    }
                                ))
                            })
                            button {
                                style: "{loadmore_style}",
                                onclick: move |_| {
                                    task_timeline.send(true);
                                },
                                "Ver más",
                            }
                        )
                    } else {
                        rsx!(
                            div {
                                class: "spinner-dual-ring--center",
                                Spinner {}
                            }
                        )
                    }
                    
                    InputMessage {
                        message_type: "text",
                        replying_to: replying_to.get().clone(),
                        is_attachable: true,
                        on_submit: on_push_message,
                        on_event: input_message_event
                    }
                    input {
                        style: "visibility: hidden;",
                        r#type: "file",
                        id: "input_file",
                        onchange: move |event: Event<FormData>| {
                            cx.spawn({
                                to_owned![task_push_attach, image_x];

                                async move {
                                    let files = &event.files;

                                    if let Some(f) = &files {
                                        let fs = f.files();
                                        let x = f.read_file(fs.get(0).unwrap()).await;
                                        
                                        if let Some(xx) = x{
                                            let y = xx.deref();
                                            image_x.set(Some(y.to_vec()));
                                            task_push_attach.send(y.to_vec());
                                        }
                                    } 
                                }
                            })
                        }
                        
                    }
                }
            }
        } else {
            rsx!(
                div {
                    class:"messages-list",
                    div {
                        style: centered,
                        Helper {
                            helper: HelperData{
                                title: String::from("Unirse a un room"),
                                description: String::from("Con este comando puedes unirte a un room indicando el id"),
                                example: String::from("!join !vaoCGcunXVlxJqWyjQ:matrix.org"),
                            }
                            on_click: x
                        }
                    }
                    InputMessage {
                        message_type: "text",
                        replying_to: &None,
                        is_attachable: false,
                        on_submit: on_push_message,
                        on_event: move |_| {}
                    }
            }
            )
        }
    })
}

pub fn handle_notification(item: NotificationItem, notification: UseState<NotificationItem>) {
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

pub async fn handle_command(message_item: MessageItem, client: &Client) {
    let query: Vec<String> = message_item
        .msg
        .trim()
        .split(' ')
        .map(|val| val.parse().unwrap())
        .collect();

    let action = query[0].as_str();
    let rid = query[1].clone();

    let room_id = RoomId::parse(rid).unwrap();

    match action {
        "!join" => join_room(client, &room_id).await,
        _ => {}
    };
}
