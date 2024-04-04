use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct Warning;
impl IconShape for Warning {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M12 8.45v4M12 21a9 9 0 1 1 0-18 9 9 0 0 1 0 18Zm.05-5.55v.1h-.1v-.1h.1Z" }
        )
    }
}
