use dioxus::prelude::*;
#[derive(Props)]
pub struct ButtonProps<'a> {
    text: &'a str,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        button {
          class: "button",
          onclick: move |event| cx.props.on_click.call(event),
          "{cx.props.text}"
      }
    ))
}
