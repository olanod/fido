use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct UserCircle;
impl IconShape for UserCircle {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M17.217 19.332A6.982 6.982 0 0 0 12 17c-2.073 0-3.935.9-5.217 2.332M12 21a9 9 0 1 1 0-18 9 9 0 0 1 0 18Zm0-7a3 3 0 1 1 0-6 3 3 0 0 1 0 6Z" }
        )
    }
}
