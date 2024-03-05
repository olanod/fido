use dioxus::prelude::*;

pub fn RoomViewSkeleton(cx: Scope) -> Element {
    cx.render(rsx! {
      div {
        class: "room-view room-view__skeleton",
        div{
          class: "avatar avatar--round avatar--skeleton skeleton",
          style: "--avatar-size: 50px",
        }
        article {
          class: "room-view-wrapper room-view-wrapper--skeleton",
          p {
            class: "room-view__title room-view__title--skeleton skeleton",
          }
          p {
            class: "room-view__message room-view__message--skeleton skeleton",
          }
        }
      }
    })
}
