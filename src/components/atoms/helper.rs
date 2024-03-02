use dioxus::prelude::*;
#[derive(Debug, Clone)]
pub struct HelperData {
    pub title: String,
    pub description: String,
    pub example: String,
}

#[derive(Props)]
pub struct HelperProps<'a> {
    helper: HelperData,
    _on_click: EventHandler<'a, MouseEvent>,
}

pub fn Helper<'a>(cx: Scope<'a, HelperProps<'a>>) -> Element<'a> {
    cx.render(rsx!(
        section {
          class: "helper",
          span {
            h3 {
                "{cx.props.helper.title}"
            },
            p {
                class: "helper__description",
                "{cx.props.helper.description}"
            },
            h4 {
                class: "helper__title",
                "Ejemplo"
            },
            p {
                class: "helper__description",
                "{cx.props.helper.example}"
            }
          }
      }
    ))
}
