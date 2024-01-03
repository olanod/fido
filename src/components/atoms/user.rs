use crate::{
    components::atoms::{header_main::HeaderCallOptions, Avatar, Close, Icon},
    services::matrix::matrix::{EventOrigin, TimelineMessageType},
};
use dioxus::prelude::*;

use super::{header_main::HeaderEvent, MessageReply};

#[derive(Props)]
pub struct UserProps<'a> {
    display_name: &'a String,
    avatar_uri: Option<&'a String>,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn UserProfile<'a>(cx: Scope<'a, UserProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
      div {
        class: "user",
        onclick: move |event| {cx.props.on_click.call(event)},

        Avatar {
          name: cx.props.display_name.to_string(),
          size: 36,
          uri: cx.props.avatar_uri.cloned()
        }

        article {
          section {
            class: "user__wrapper",
            span {
              class: "user__content",
              "{cx.props.display_name}"
            }
          }
        }
      }
    })
}
