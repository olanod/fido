use dioxus::prelude::*;
use gloo::events::EventListener;
use web_sys::HtmlElement;

use crate::{components::{atoms::{
    message::Message,
    *,
}, molecules::input_message::ReplyingTo}, utils::get_element::GetElement, hooks::use_messages::{use_messages, UseMessages}, pages::chat::chat::ListHeight};

use super::rooms::CurrentRoom;

pub fn List(cx: Scope) -> Element {
    let use_m = use_messages(cx);
    let UseMessages {messages, isLoading: _, limit: _, task: _} = use_m.get();

    let replying_to = use_shared_state::<Option<ReplyingTo>>(cx).unwrap();
    let height = use_shared_state::<ListHeight>(cx).unwrap();
    let container_to_scroll = use_ref::<Option<HtmlElement>>(cx, || None);
    let list_to_scroll = use_ref::<Option<HtmlElement>>(cx, || None);
    let current_room = use_shared_state::<CurrentRoom>(cx).unwrap();

    cx.render(rsx! {
        div {
            style: "{height.read().height}",
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
