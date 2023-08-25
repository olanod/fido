use dioxus::prelude::*;

use super::icon::IconShape;

pub struct ArrowLeft;
impl IconShape for ArrowLeft {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")    
    }
    fn child_elements(&self) -> LazyNodes {
        rsx!(path {
            d: "M19 12H5m0 0 6 6m-6-6 6-6"
        })
    }
}
