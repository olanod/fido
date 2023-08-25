use dioxus::prelude::*;

use super::icon::IconShape;

pub struct Close;
impl IconShape for Close {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")    
    }
    fn child_elements(&self) -> LazyNodes {
        rsx!(path {
            d: "m16 16-4-4m0 0L8 8m4 4 4-4m-4 4-4 4"
        })
    }
}
