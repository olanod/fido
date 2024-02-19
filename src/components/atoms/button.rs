use dioxus::prelude::*;

pub enum Variant {
    Primary,
    Secondary,
}

#[derive(Props)]
pub struct ButtonProps<'a> {
    text: &'a str,
    #[props(default = &Variant::Primary)]
    variant: &'a Variant,
    #[props(default = false)]
    disabled: bool,
    on_click: EventHandler<'a, MouseEvent>,
    #[props(!optional)]
    status: Option<String>,
}

pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let variant = match cx.props.variant {
        Variant::Primary => "button--primary",
        Variant::Secondary => "button--secondary",
    };

    let disabled = if cx.props.disabled {
        "button--disabled"
    } else {
        ""
    };

    let loading = if cx.props.status.is_some() {
        "button--loading"
    } else {
        ""
    };

    match &cx.props.status {
        Some(s) => {
            render!(rsx!(
                button {
                  class: "button {variant} {loading}",
                  disabled: true,
                  "{s}"
              }
            ))
        }
        None => {
            render!(rsx!(
                button {
                  class: "button {variant} {disabled}",
                  disabled: cx.props.disabled,
                  onclick: move |event| cx.props.on_click.call(event),
                  "{cx.props.text}"
              }
            ))
        }
    }
}
