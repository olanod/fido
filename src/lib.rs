#![allow(non_snake_case)]

use matrix_sdk::Client;
pub mod components {
    pub mod atoms;
    pub mod molecules;
    pub mod organisms;
}

pub mod services {
    pub mod matrix;
}

pub mod utils {
    pub mod matrix;
}

pub mod pages {
    pub mod home;
}

pub struct MatrixClientState {
    pub client: Option<Client>,
}
