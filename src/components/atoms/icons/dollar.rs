use dioxus::prelude::*;

use super::icon::IconShape;

pub struct Dollar;
impl IconShape for Dollar {
    fn view_box(&self) -> String {
        String::from("0 0 20 20")
    }

    fn child_elements(&self) -> LazyNodes {
        rsx!(path {
            d: "M10 1.75v16.5m3.75-13.5H8.125a2.625 2.625 0 0 0 0 5.25h3.75a2.625 2.625 0 0 1 0 5.25H5.5"
        })
    }
}
