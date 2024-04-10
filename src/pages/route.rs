use dioxus::prelude::*;
use dioxus_router::prelude::*;

use super::{page_not_found::PageNotFound, profile::profile::Profile, profile::verify::Verify};

use crate::{
    pages::chat::chat::Chat, pages::chat::chat_list::ChatList, pages::chat::room::group::RoomGroup,
    pages::chat::room::new::RoomNew,
};

use crate::components::organisms::IndexMenu;

/// An enum of all of the possible routes in the app.
#[derive(Routable, Clone, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(IndexMenu)]
        #[route("/profile")]
        Profile {},
        #[route("/verify/:id")]
        Verify {id: String},
        #[route("/")]
        #[layout(Chat)] 
            #[route("/list")]
            ChatList {},
        #[end_layout]
        #[route("/room")]
        RoomNew {},
        #[route("/group")]
        RoomGroup {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
