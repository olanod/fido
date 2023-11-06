use dioxus::prelude::*;

use super::icon::IconShape;

pub struct Attachment;
impl IconShape for Attachment {
    fn view_box(&self) -> String {
        String::from("0 0 20 20")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx!(path {
            d: "m17.08 9.287-6.892 6.893A4.502 4.502 0 1 1 3.82 9.813l6.893-6.893a3.002 3.002 0 1 1 4.245 4.245l-6.9 6.892a1.5 1.5 0 1 1-2.123-2.122l6.368-6.36"
        })
    }
}
