use dioxus::prelude::*;

use super::icon::IconShape;

pub struct Attachment;
impl IconShape for Attachment {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> LazyNodes {
        rsx!(path {
            d: "M4.536 11.465 11.43 4.57a5.25 5.25 0 1 1 7.424 7.425L10.9 19.95A3.5 3.5 0 0 1 5.95 15l7.956-7.955A1.75 1.75 0 0 1 16.38 9.52l-6.895 6.894"
        })
    }
}
