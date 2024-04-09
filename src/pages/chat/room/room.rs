use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::route::Route;

pub fn Room() -> Element {
    rsx! { Outlet::<Route> {} }
}
