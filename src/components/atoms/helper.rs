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
    on_click: EventHandler<'a, MouseEvent>,
}

pub fn Helper<'a>(cx: Scope<'a, HelperProps<'a>>) -> Element<'a> {
    let helper_style = r#"
        width: fit-content;
        height: fit-content;
        min-height: 100px;
        min-width: 150px;
        max-width: 390px;
        color: var(--text-1);
        border-radius: var(--size-1);
        padding: var(--size-3);
        transition: opacity 0.2s ease-out, background-color 0.2s ease-out;
        background: white;
        text-align: left;
        border: 1px solid transparent;
    "#;
    cx.render(rsx!(
        section {
          style: helper_style,
          span {
            h3 {
                "{cx.props.helper.title}"
            },
            p {
                style: r"
                padding-top: var(--size-0);
            ",
                "{cx.props.helper.description}"
            },
            h4 {
                style: r"
                padding-top: var(--size-2);
            ",
                "Ejemplo"
            },
            p {
                style: r"
                padding-top: var(--size-0);
            ",
                "{cx.props.helper.example}"
            }
          }
      }
    ))
}
