#![allow(non_snake_case)]

use matrix_sdk::Client;
pub mod components {
    pub mod atoms;
    pub mod molecules;
    pub mod organisms;
}

pub mod hooks {
    pub mod use_attach;
    pub mod use_client;
    pub mod use_init_app;
    pub mod use_listen_message;
    pub mod use_messages;
    pub mod use_modal;
    pub mod use_notification;
    pub mod use_room;
    pub mod use_send_attach;
    pub mod use_send_message;
}

pub mod services {
    pub mod matrix;
}

pub mod utils {
    pub mod get_element;
    pub mod i18n_get_key_value;
    pub mod matrix;
}

pub mod pages {
    pub mod chat;
    pub mod login;
    pub mod page_not_found;
    pub mod profile;
    pub mod route;
}

#[derive(Clone)]
pub struct MatrixClientState {
    pub client: Option<Client>,
}
