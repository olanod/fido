use dioxus::prelude::*;

use crate::components::atoms::{helper::HelperData, Helper};

// #[derive(Debug)]
// pub struct FormRoomEvent {
//    pub room: ,
// }

#[derive(Props)]
pub struct HelpersListProps<'a> {
    helpers: &'a Vec<HelperData>,
    // on_submit: EventHandler<'a, FormRoomEvent>,
}

pub fn HelpersList<'a>(cx: Scope<'a, HelpersListProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div{
            style: r"
                display: grid;
                grid-template-columns: 3;
                grid-template-rows: 1;
                width: 50%;
                height: 50%;
            ",
            cx.props.helpers.iter().map(|room| {
                rsx!(Helper {
                    helper: HelperData{
                        title: String::from("Unirse a un room"),
                        description: String::from("Con este comando puedes unirte a un room indicando el id"),
                        example: String::from("!join alsdkfjlaksdjflkjalksdjf"),
                    }
                    on_click: move |_| {

                    }
                })
            })
        }
    })
}
