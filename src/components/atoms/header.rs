use dioxus::prelude::{SvgAttributes, *};

#[derive(Debug)]
pub enum HeaderCallOptions {
    CLOSE,
}

#[derive(Debug)]
pub struct HeaderEvent {
    pub value: HeaderCallOptions,
}

#[derive(Props)]
pub struct HeaderProps<'a> {
    text: &'a str,
    on_event: EventHandler<'a, HeaderEvent>,
}

pub fn Header<'a>(cx: Scope<'a, HeaderProps<'a>>) -> Element<'a> {
    let nav_style = r#"
        color: var(--text-1);
        display: flex;
        justify-content: space-between;
        align-items: center;
        position: absolute;
        width: inherit;
        height: 40px;
        padding: 0 var(--size-4);
        background: var(--surface-1);
        font-weight: 600;
        top: 0;
        font-size: var(--font-size-0)
    "#;

    let close_style = r#"
      cursor: pointer;
      background: transparent;
      border: 1px solid transparent;
    "#;

    let icon_style = r#"
      fill: var(--text-1)
    "#;

    cx.render(rsx!(
        nav {
          style: "{nav_style}",
          span {
            "{cx.props.text}"
          }
          button {
            style: "{close_style}",
            onclick: move |_| {cx.props.on_event.call(HeaderEvent { value: HeaderCallOptions::CLOSE })},
            svg {
              style: "{icon_style}",
              view_box: "0 0 50 50",
              height: 20,
              width: 20,
              path {
                  d: "M 9.15625 6.3125 L 6.3125 9.15625 L 22.15625 25 L 6.21875 40.96875 L 9.03125 43.78125 L 25 27.84375 L 40.9375 43.78125 L 43.78125 40.9375 L 27.84375 25 L 43.6875 9.15625 L 40.84375 6.3125 L 25 22.15625 Z"
              }
            }
          }
      }
    ))
}
