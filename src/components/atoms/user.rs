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
    let header_style = r#"
        display: flex;
        justify-content: space-between;
    "#;

    let sender_style = r#"
        color: var(--text-1);
        font-weight: 500;
    "#;

    let content_style = r#"
        font-size: var(--font-size-0);
        display: flex;
        gap: 11px;
        align-items: flex-end;
        justify-content: space-between;
    "#;

    let origin_message_container_style = r#"
      border-radius: 16px;
      border: 0.5px solid var(--border-normal-50);
      background: var(--background-loud);
      color: var(--text-white);
      display: inline-block;
      width: fit-content;
      max-width: 80%;
      margin: 0 var(--size-1) 0 auto;
    "#;

    cx.render(rsx! {
      div {
        style: "{origin_message_container_style}",
        onclick: move |event| {cx.props.on_click.call(event)},

        Avatar {
          name: cx.props.display_name.to_string(),
          size: 36,
          uri: cx.props.avatar_uri.cloned()
        }

        article {
          section {
            style: "{header_style}",
            span {
              style: "{sender_style}",
              "{cx.props.display_name}"
            }
          }
        }
      }
    })
}
