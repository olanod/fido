use dioxus::prelude::*;

use super::icon::IconShape;

pub struct Chart;
impl IconShape for Chart {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }

    fn child_elements(&self) -> LazyNodes {
        rsx!(path {
            d: "M12 3a9 9 0 1 0 9 9m-9-9a9 9 0 0 1 9 9m-9-9v9m9 0h-9m6 6.5L12 12"
        })
    }
}
