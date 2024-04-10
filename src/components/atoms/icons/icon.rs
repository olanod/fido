use dioxus::prelude::*;

pub trait IconShape: PartialEq {
    fn view_box(&self) -> String;
    fn child_elements(&self) -> Element;
}

#[derive(PartialEq, Clone, Props)]
pub struct IconProps<T: IconShape + 'static> {
    #[props(default = 20)]
    pub height: u32,
    #[props(default = 20)]
    pub width: u32,
    #[props(default = "none".to_string())]
    pub fill: String,
    #[props(default = "none".to_string())]
    pub stroke: String,
    #[props(default = "2".to_string())]
    pub stroke_width: String,
    #[props(default = "".to_string())]
    pub class: String,
    pub icon: T,
}

pub fn Icon<T: IconShape + 'static>(props: IconProps<T>) -> Element {
    let icon_style = format!(
        r#"
        width: 100%;
        max-width: {}px;
    "#,
        props.width
    );

    rsx! {
        svg {
            style: "{icon_style}",
            stroke: props.stroke,
            stroke_width: props.stroke_width,
            class: format_args!("{}", props.class),
            height: format_args!("{}", props.height),
            fill: format_args!("{}", props.fill),
            view_box: format_args!("{}", props.icon.view_box()),
            stroke_linecap: "round",
            stroke_linejoin: "round",
            {props.icon.child_elements()}
        }
    }
}
