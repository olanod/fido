use dioxus::prelude::*;

// mod components {
//     pub mod atoms;
// }

// use components::atoms::IndexAtoms;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
      h1{
        "Home"
      }
    })
}
