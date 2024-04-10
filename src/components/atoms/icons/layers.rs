use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct Layers;
impl IconShape for Layers {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }

    fn child_elements(&self) -> Element {
        rsx!( path { d: "m21 12-9 6-9-6m18 4-9 6-9-6m18-8-9 6-9-6 9-6 9 6Z" } )
    }
}
