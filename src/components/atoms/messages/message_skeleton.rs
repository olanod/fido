use dioxus::prelude::*;

use crate::services::matrix::matrix::EventOrigin;

#[derive(PartialEq, Props)]
pub struct MessageSkeletonProps {
    pub origin: EventOrigin,
}

pub fn MessageSkeleton(cx: Scope<MessageSkeletonProps>) -> Element {
    let dropdown_left = match cx.props.origin {
        EventOrigin::ME => "",
        EventOrigin::OTHER => "dropdown--left",
    };

    render!(rsx! {
      div {
        class: "dropdown {dropdown_left} dropdown--skeleton",
        div {
          class: "message-view",
          // Header content (Avatar)
          match &cx.props.origin {
            EventOrigin::ME => None,
            EventOrigin::OTHER => render!(
                rsx!(
                    div{
                        class: "avatar avatar--round avatar--skeleton skeleton",
                        style: "--avatar-size: 36px",
                    }
                )
            )
          }
          article {
            class: "message-wrapper message-wrapper--skeleton",
            // Name sender content
            match cx.props.origin {
              EventOrigin::OTHER =>
              render!(
                rsx!(
                  p {
                    class: "message__sender message__sender--skeleton skeleton"
                  }
                )
              ),
              _ => None
            }

            div {
                class: "message__container__content",
                div{
                    class: "message__content message__content--skeleton",
                    p {
                        class: "message__content message__content--skeleton skeleton-text skeleton",
                    }
                }
                span {
                    class: "message__time message__time--skeleton skeleton-text skeleton",
                }
            }
          }
        }
      }
    })
}
