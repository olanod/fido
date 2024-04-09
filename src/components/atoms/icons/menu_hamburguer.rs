use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct MenuHamburger;
impl IconShape for MenuHamburger {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!( path { d: "M5 17h14M5 12h14M5 7h14" } )
    }
}
