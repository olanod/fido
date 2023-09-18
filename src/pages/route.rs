use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::{profile::profile::Profile, page_not_found::PageNotFound};

use crate::{
    pages::chat::chat::Chat, pages::chat::chat_list::ChatList, pages::chat::chat_room::ChatRoom,
};

use crate::components::organisms::IndexMenu;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(IndexMenu)]
        #[route("/profile")]
        Profile {},
        #[route("/")]
        #[layout(Chat)] 
            #[route("/list")]
            ChatList {},
            #[route("/room/:name")]
            ChatRoom {name: String},
        #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
