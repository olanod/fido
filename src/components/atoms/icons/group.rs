use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct Group;
impl IconShape for Group {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }

    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M17 20c0-1.657-2.239-3-5-3s-5 1.343-5 3m14-3c0-1.23-1.234-2.287-3-2.75M3 17c0-1.23 1.234-2.287 3-2.75m12-4.014a3 3 0 1 0-4-4.472m-8 4.472a3 3 0 0 1 4-4.472M12 14a3 3 0 1 1 0-6 3 3 0 0 1 0 6Z" }
        )
    }
}
