use std::ops::Deref;

use dioxus::prelude::*;
use gloo::events::EventListener;
use log::info;
use web_sys::HtmlElement;

use crate::components::atoms::message::Sender;
use crate::components::atoms::message::ThreadPreview;
use crate::services::matrix::matrix::TimelineMessage;
use crate::services::matrix::matrix::TimelineRelation;
use crate::services::matrix::matrix::TimelineThread;
use crate::{components::{atoms::{
    message::Message,
    *, messages::hover_menu::{MenuEvent, MenuOption},
}, molecules::input_message::ReplyingTo}, utils::get_element::GetElement, hooks::use_messages::{use_messages, UseMessages}, pages::chat::chat::ListHeight, services::matrix::matrix::{TimelineMessageType, ImageType}};

use super::rooms::CurrentRoom;

pub fn List(cx: Scope) -> Element {
    let use_m = use_messages(cx);
    let UseMessages {messages, isLoading: _, limit: _, task: _} = use_m.get();

    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();
    let height = use_shared_state::<ListHeight>(cx).unwrap();
    let container_to_scroll = use_ref::<Option<HtmlElement>>(cx, || None);
    let list_to_scroll = use_ref::<Option<HtmlElement>>(cx, || None);
    let current_room = use_shared_state::<CurrentRoom>(cx).unwrap();

    let timeline_thread = use_shared_state::<Option<TimelineThread>>(cx).unwrap();
    let list_style = match timeline_thread.read().deref() {
        Some(_)=> "
            background: var(--background-modal);
            padding: 12px;
            border-radius: 16px;
        ",
        None => ""
    };

    cx.render(rsx! {
        if let Some(t) = timeline_thread.read().deref() {
            let head_message = &t.thread[t.thread.len() - 1];
            let x = &head_message.body;
            
            let title_thread = match x {
                TimelineMessageType::Image(file) => {
                    String::from(file.body.clone())
                },
                TimelineMessageType::Text(text) => {
                    String::from(text.clone())
                },
                TimelineMessageType::Html(html) => {
                    String::from(html.clone())
                },
                TimelineMessageType::File(file) => {
                    String::from(file.body.clone())
                },
                TimelineMessageType::Video(file) => {
                    String::from(file.body.clone())
                },
                TimelineMessageType::Payment(p) => {
                    String::from("Pago")
                },
            };

            let close_style = r#"
              cursor: pointer;
              background: transparent;
              border: 1px solid transparent;
              display: flex;
            "#;

            rsx!(
                div {
                    style: "
                        padding: 12px;
                        background: var(--background-modal);
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        width: 100%;
                        position: absolute;
                        left: 0;
                        border-radius: 16px 16px 0;
                        z-index: 100;
                    ",
                    
                    button {
                        style: "{close_style}",
                        onclick: move |_| {
                            *timeline_thread.write() = None
                        },
                        Icon {
                          stroke: "var(--icon-subdued)",
                          icon: ArrowLeft
                        }
                      }
                    p {
                        style: "
                            overflow: hidden;
                            display: -webkit-box;
                            -webkit-line-clamp: 1;
                            -webkit-box-orient: vertical;
                        ",
                        "Hilo {title_thread}"
                    }
                    button {
                        style: "{close_style}",
                        onclick: move |_| {
                            *timeline_thread.write() = None
                        },
                        Icon {
                          stroke: "var(--icon-subdued)",
                          icon: Close
                        }
                      }
                }
            )
        }
        
        div {
            style: "
                {height.read().height}
                {list_style}
            ",
            class:"messages-list",
            id: "messages-container",
            onmounted: move |_| {
                container_to_scroll.set(Some(GetElement::<web_sys::HtmlElement>::get_element_by_id("messages-container")));
            },
            
            // if !isLoading {
                rsx!(
                    div {
                        style: "
                            display: flex;
                            flex-direction: column-reverse;
                            background: var(--background);
                            height: 100vh;
                            border-radius: 8px;
                        ",
                        id: "listx",
                        onmounted: move |_| {
                            let lista = GetElement::<web_sys::HtmlElement>::get_element_by_id("listx");
                            list_to_scroll.set(Some(lista.clone()));
                            
                            let current_room_id = current_room.read().id.clone();
                            to_owned!(use_m);
                            

                            if let Some(contenedor) = container_to_scroll.read().clone() {
                                let contenedor = contenedor.clone();
                                let lista = lista.clone();
                                
                                let use_m = use_m.clone();

                                let mut old_value = 0;

                                let on_down = EventListener::new(&contenedor.clone(), "scroll", move |_| {
                                    let alturaContenedor = contenedor.client_height();
                                    let alturaScroll = contenedor.scroll_top() * -1; 
                                    let alturaLista = lista.client_height();
                                    
                                    let x = alturaLista * 80 / 100;
                                    
                                    if alturaContenedor + alturaScroll >= x && alturaScroll > old_value && !use_m.get().isLoading  {
                                            use_m.loadmore(current_room_id.clone());
                                    }

                                    old_value = alturaScroll;
                                });
                                on_down.forget();
                            }
                        },
                        match timeline_thread.read().deref() {
                            Some(final_thread) => {
                                rsx!(
                                    final_thread.thread.iter().enumerate().map(|(i, m)| {
                                        let message = m.clone();
                                        let event_id = message.event_id.clone();

                                        cx.render(rsx!(
                                            MessageView {
                                                key: "{i}",
                                                message: Message {
                                                    id: i as i64,
                                                    event_id: message.event_id,
                                                    display_name: message.sender.name.clone(),
                                                    avatar_uri: message.sender.avatar_uri.clone(),
                                                    content: message.body.clone(),
                                                    reply: None,
                                                    origin: message.origin.clone(),
                                                    time: message.time.clone(),
                                                    thread: None
                                                },
                                                is_replying: false,
                                                on_event: move |event: MenuEvent| {
                                                    info!("menu option list: {:?}", event.option);
    
                                                    match event.option {
                                                        MenuOption::Download => {
                                                            info!("TODO: handle download")
                                                        }
                                                        MenuOption::Reply => {
                                                            let height = height.clone();
                                                            if let Some(eid) = &event_id {
                                                                let replying = ReplyingTo { 
                                                                    event_id: eid.clone(), 
                                                                    content: message.body.clone(), 
                                                                    display_name: message.sender.name.clone(), 
                                                                    avatar_uri: message.sender.avatar_uri.clone(),
                                                                    origin: message.origin.clone()
                                                                };
                                                                
                                                                *replying_to.write() = Some(replying);
                                                            
                                                                let element = GetElement::<web_sys::HtmlElement>::get_element_by_id("input_field");
                        
                                                                info!("into event list");
                                                                gloo::timers::callback::Timeout::new(250, move || {      
                                                                    let h = element.offset_height();
                                                                    let x = format!("height: calc(100vh - 72px - {}px );", h + 18); 
                                                                    height.write().height = x;
                                                                })
                                                                .forget();
                                                            }
                                                        }
                                                        MenuOption::Close => {
                                                            info!("close");
                                                        }
                                                        MenuOption::ShowThread => {
                                                            info!("thread");
                                                        }
                                                        MenuOption::CreateThread => {
                                                            info!("thread");
                                                        }
                                                    }
                                                    
                                                }
                                            }
                                        ))
                                    })
                                )
                            }
                            None => {
                                rsx!(
                                    messages.iter().enumerate().map(|(i, m)| {
                                        match m {
                                            TimelineRelation::None(message) => {
                                                let message = message.clone();
                                                let event_id = message.event_id.clone();
            
                                                cx.render(rsx!(
                                                    MessageView {
                                                        key: "{i}",
                                                        message: Message {
                                                            id: i as i64,
                                                            event_id: message.event_id,
                                                            display_name: message.sender.name.clone(),
                                                            avatar_uri: message.sender.avatar_uri.clone(),
                                                            content: message.body.clone(),
                                                            reply: None,
                                                            origin: message.origin.clone(),
                                                            time: message.time.clone(),
                                                            thread: None
                                                        },
                                                        is_replying: false,
                                                        on_event: move |event: MenuEvent| {
                                                            
                                                            info!("menu option list: {:?}", event.option);
            
                                                            match event.option {
                                                                MenuOption::Download => {
                                                                    info!("TODO: handle download")
                                                                }
                                                                MenuOption::Reply => {
                                                                    let height = height.clone();
                                                                    if let Some(eid) = &event_id {
                                                                        let replying = ReplyingTo { 
                                                                            event_id: eid.clone(), 
                                                                            content: message.body.clone(), 
                                                                            display_name: message.sender.name.clone(), 
                                                                            avatar_uri: message.sender.avatar_uri.clone(),
                                                                            origin: message.origin.clone()
                                                                        };
                                                                        
                                                                        *replying_to.write() = Some(replying);
                                                                    
                                                                        let element = GetElement::<web_sys::HtmlElement>::get_element_by_id("input_field");
                                
                                                                        info!("into event list");
                                                                        gloo::timers::callback::Timeout::new(250, move || {      
                                                                            let h = element.offset_height();
                                                                            let x = format!("height: calc(100vh - 72px - {}px );", h + 18); 
                                                                            height.write().height = x;
                                                                        })
                                                                        .forget();
                                                                    }
                                                                }
                                                                MenuOption::Close => {
                                                                    info!("close");
                                                                }
                                                                MenuOption::ShowThread => {
                                                                    info!("thread");
                                                                }
                                                                MenuOption::CreateThread => {
                                                                    info!("create thread");
                                                                    let thread = vec![TimelineMessage { event_id: event_id.clone(), sender: message.sender.clone(), body: message.body.clone(), origin: message.origin.clone(), time: message.time.clone() }];
                                                                    if let Some(e) = &event_id {
                                                                        *timeline_thread.write() = Some(TimelineThread { event_id: e.clone(), thread, count: 0, latest_event: e.clone() })
                                                                    }
                                                                }

                                                            }
                                                            
                                                        }
                                                    }
                                                ))
                                            }
                                            TimelineRelation::Reply(message) => {
                                                let r = message.reply.clone();
                                                let message = message.event.clone();
                                                let event_id = message.event_id.clone();

                                                let reply = match r {
                                                    Some(r) => {
                                                        Some(MessageReply {
                                                            content: r.body,
                                                            display_name: r.sender.name,
                                                            avatar_uri: r.sender.avatar_uri
                                                        })
                                                    }
                                                    None => {
                                                        None
                                                    }
                                                };
            
                                                cx.render(rsx!(
                                                    MessageView {
                                                        key: "{i}",
                                                        message: Message {
                                                            id: i as i64,
                                                            event_id: message.event_id,
                                                            display_name: message.sender.name.clone(),
                                                            avatar_uri: message.sender.avatar_uri.clone(),
                                                            content: message.body.clone(),
                                                            reply,
                                                            origin: message.origin.clone(),
                                                            time: message.time.clone(),
                                                            thread: None
                                                        },
                                                        is_replying: false,
                                                        on_event: move |event: MenuEvent| {
                                                            info!("menu option list: {:?}", event.option);
            
                                                            match event.option {
                                                                MenuOption::Download => {
                                                                    info!("TODO: handle download")
                                                                }
                                                                MenuOption::Reply => {
                                                                    let height = height.clone();
                                                                    if let Some(eid) = &event_id {
                                                                        let replying = ReplyingTo { 
                                                                            event_id: eid.clone(), 
                                                                            content: message.body.clone(), 
                                                                            display_name: message.sender.name.clone(), 
                                                                            avatar_uri: message.sender.avatar_uri.clone(),
                                                                            origin: message.origin.clone()
                                                                        };
                                                                        
                                                                        *replying_to.write() = Some(replying);
                                                                    
                                                                        let element = GetElement::<web_sys::HtmlElement>::get_element_by_id("input_field");
                                
                                                                        info!("into event list");
                                                                        gloo::timers::callback::Timeout::new(250, move || {      
                                                                            let h = element.offset_height();
                                                                            let x = format!("height: calc(100vh - 72px - {}px );", h + 18); 
                                                                            height.write().height = x;
                                                                        })
                                                                        .forget();
                                                                    }
                                                                }
                                                                MenuOption::Close => {
                                                                    info!("close");
                                                                }
                                                                MenuOption::ShowThread => {
                                                                    info!("thread");
                                                                }
                                                                MenuOption::CreateThread => {
                                                                    
                                                                }
                                                            }
                                                            
                                                        }
                                                    }
                                                ))
                                            }
                                            TimelineRelation::CustomThread(message) => {
                                                let event_id = message.event_id.clone();
                                                let thread = message.thread.clone();
                                                let latest_event = message.latest_event.clone();
                                                let count = message.count.clone();
                                                let head_message = thread[thread.len() - 1].clone();
                                                
                                                let mut thread_avatars: Vec<Sender> = vec![];
            
                                                for (i, t) in thread.iter().enumerate() {
                                                    if i == 2 {
                                                        break;
                                                    }
            
                                                    thread_avatars.push(Sender{avatar_uri: t.sender.avatar_uri.clone(), display_name: t.sender.name.clone()})
                                                }
            
                                                cx.render(rsx!(
                                                    MessageView {
                                                        key: "{i}",
                                                        message: Message {
                                                            id: i as i64,
                                                            event_id: head_message.event_id.clone(),
                                                            display_name: head_message.sender.name.clone(),
                                                            avatar_uri: head_message.sender.avatar_uri.clone(),
                                                            content: head_message.body.clone(),
                                                            reply: None,
                                                            origin: head_message.origin.clone(),
                                                            time: head_message.time.clone(),
                                                            thread: Some(ThreadPreview{meta_senders: thread_avatars, count: (thread.len() - 1) as i8 })
                                                        },
                                                        is_replying: false,
                                                        on_event: move |event: MenuEvent| {
                                                            info!("menu option list: {:?}", event.option);
            
                                                            match event.option {
                                                                MenuOption::Download => {
                                                                    info!("TODO: handle download")
                                                                }
                                                                MenuOption::Reply => {
                                                                    let height = height.clone();
                                                                    let replying = ReplyingTo { 
                                                                        event_id: event_id.clone(), 
                                                                        content: head_message.body.clone(), 
                                                                        display_name: head_message.sender.name.clone(), 
                                                                        avatar_uri: head_message.sender.avatar_uri.clone(),
                                                                        origin: head_message.origin.clone()
                                                                    };
                                                                    
                                                                    *replying_to.write() = Some(replying);
                                                                
                                                                    let element = GetElement::<web_sys::HtmlElement>::get_element_by_id("input_field");
                            
                                                                    info!("into event list");
                                                                    gloo::timers::callback::Timeout::new(250, move || {      
                                                                        let h = element.offset_height();
                                                                        let x = format!("height: calc(100vh - 72px - {}px );", h + 18); 
                                                                        height.write().height = x;
                                                                    })
                                                                    .forget();
                                                                }
                                                                MenuOption::Close => {
                                                                    info!("close");
                                                                }
                                                                MenuOption::ShowThread => {
                                                                    info!("thread");
                                                                    *timeline_thread.write() = Some(TimelineThread { event_id: event_id.clone(), thread: thread.clone(), count: count.clone(), latest_event: latest_event.clone() })
                                                                }
                                                                MenuOption::CreateThread => {

                                                                }
                                                            }
                                                            
                                                        }
                                                    }
                                                ))
                                            }
                                            TimelineRelation::Thread(_) => {
                                                cx.render(rsx!(
                                                    div {}
                                                ))
                                            }
                                        }
                                    })
                                )
                            }
                        }
                    }

                    
                )
            // } else {
            //     rsx!(
            //         div {
            //             class: "spinner-dual-ring--center",
            //             Spinner {}
            //         }
            //     )
            // }
        }
    })
}
