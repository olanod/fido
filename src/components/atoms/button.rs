use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Variant {
    Primary,
    Secondary,
    Tertiary,
}

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    text: String,
    #[props(default = Variant::Primary)]
    variant: Variant,
    #[props(default = false)]
    disabled: bool,
    on_click: EventHandler<MouseEvent>,
    #[props(!optional)]
    status: Option<String>,
}

pub fn Button(props: ButtonProps) -> Element {
    let variant = match props.variant {
        Variant::Primary => "button--primary",
        Variant::Secondary => "button--secondary",
        Variant::Tertiary => "button--tertiary",
    };

    let disabled = if props.disabled {
        "button--disabled"
    } else {
        ""
    };

    let loading = if props.status.is_some() {
        "button--loading"
    } else {
        ""
    };

    match &props.status {
        Some(s) => {
            rsx!( button { class: "button {variant} {loading}", disabled: true, "{s}" } )
        }
        None => {
            rsx!(
                button {
                    class: "button {variant} {disabled}",
                    disabled: props.disabled,
                    onclick: move |event| props.on_click.call(event),
                    "{props.text}"
                }
            )
        }
    }
}
