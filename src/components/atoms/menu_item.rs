use dioxus::prelude::*;

#[derive(Props)]
pub struct MenuItemProps<'a> {
    #[props(!optional)]
    title: &'a str,
    icon: Element<'a>,
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn MenuItem<'a>(cx: Scope<'a, MenuItemProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
      div {
        class: "room-view",
        onclick: move |event| cx.props.on_click.call(event),

        &cx.props.icon
        p {
          class: "room-view__title",
          "{cx.props.title}"
        }
      }
    })
}
