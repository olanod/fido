use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct Reply;
impl IconShape for Reply {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }

    fn child_elements(&self) -> Element {
        rsx!( path { d: "M7 13 3 9m0 0 4-4M3 9h13a5 5 0 0 1 0 10h-5" } )
    }
}
