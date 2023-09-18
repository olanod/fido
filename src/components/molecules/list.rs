use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_std::{translate, i18n::use_i18};

use crate::{components::{atoms::{
    message::Message,
    *,
}, molecules::input_message::ReplyingTo}, utils::{i18n_get_key_value::i18n_get_key_value, get_element::GetElement}, hooks::use_messages::{use_messages, UseMessages}, pages::chat::chat::ListHeight};

use super::rooms::CurrentRoom;

pub fn List(cx: Scope) -> Element {
    let i18 = use_i18(cx);

    let i18n_map = HashMap::from([
        ("message-list-see-more", translate!(i18, "chat.message_list.see_more")),
    ]);

    let use_m = use_messages(cx);
    let UseMessages {messages, isLoading, limit: _ } = use_m.get();

    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();
    let height = use_shared_state::<ListHeight>(cx).unwrap();

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
        margin: 0.5rem auto 0;
    "#;
    
    cx.render(rsx! {
        div {
            style: "{height.read().height}",
            class:"messages-list",
            if !isLoading {
                rsx!(
                    messages.iter().map(|message| {
                        let message = message.clone();
                        let event_id = message.event_id.clone();
                        

                        cx.render(rsx!(
                            MessageView {
                                key: "{message.id}",
                                message: Message {
                                    id: message.id,
                                    event_id: message.event_id,
                                    display_name: message.display_name.clone(),
                                    avatar_uri: message.avatar_uri.clone(),
                                    content: message.content.clone(),
                                    reply: message.reply.clone(),
                                    origin: message.origin.clone(),
                                    time: message.time.clone()
                                },
                                is_replying: false,
                                on_event: move |_| {
                                    let height = height.clone();
                                    if let Some(eid) = &event_id {
                                        let replying = ReplyingTo { 
                                            event_id: eid.clone(), 
                                            content: message.content.clone(), 
                                            display_name: message.display_name.clone(), 
                                            avatar_uri: message.avatar_uri.clone(),
                                            origin: message.origin.clone()
                                        };
                                        
                                        *replying_to.write() = Some(replying);
                                      
                                        let element = GetElement::<web_sys::HtmlElement>::get_element_by_id("input_field");

                                        gloo::timers::callback::Timeout::new(50, move || {      
                                            let h = element.offset_height();
                                            let x = format!("height: calc(100vh - 72px - {}px );", h + 18); 
                                            height.write().height = x;
                                        })
                                        .forget();
                                    }
                                }
                            }
                        ))
                    })
                    button {
                        style: "{loadmore_style}",
                        onclick: move |_| {
                            let current_room = use_shared_state::<CurrentRoom>(cx).unwrap();
                            let current_room_id = current_room.read().id.clone();

                            use_m.loadmore(current_room_id)
                        },
                        i18n_get_key_value(&i18n_map, "message-list-see-more"),
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
        }
    })
}
