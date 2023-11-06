use dioxus::prelude::*;

use super::icon::IconShape;

pub struct ChevronRight;
impl IconShape for ChevronRight {
    fn view_box(&self) -> String {
        String::from("0 0 20 20")
    }

    fn child_elements(&self) -> LazyNodes {
        rsx!(path {
            d: "m8.2 13.6 3.6-3.6-3.6-3.6"
        })
    }
}
