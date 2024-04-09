use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct Search;
impl IconShape for Search {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }

    fn child_elements(&self) -> Element {
        rsx!( path { d: "m15 15 6 6m-11-4a7 7 0 1 1 0-14 7 7 0 0 1 0 14Z" } )
    }
}
