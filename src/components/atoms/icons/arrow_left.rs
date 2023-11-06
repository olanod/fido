use dioxus::prelude::*;

use super::icon::IconShape;

pub struct ArrowLeft;
impl IconShape for ArrowLeft {
    fn view_box(&self) -> String {
        String::from("0 0 20 20")    
    }
    fn child_elements(&self) -> LazyNodes {
        rsx!(path {
            d: "M15.25 10H4.75M10 15.25 4.75 10 10 4.75"
        })
    }
}
