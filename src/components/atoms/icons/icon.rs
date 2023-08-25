use dioxus::prelude::*;

pub trait IconShape {
    fn view_box(&self) -> String;
    fn child_elements(&self) -> LazyNodes;
}

#[derive(Props)]
pub struct IconProps<'a, T: IconShape> {
    #[props(default = 20)]
    pub height: u32,
    #[props(default = 20)]
    pub width: u32,
    #[props(default = "none")]
    pub fill: &'a str,
    #[props(default = "none")]
    pub stroke: &'a str,
    #[props(default = "2")]
    pub stroke_width: &'a str,
    #[props(default = "")]
    pub class: &'a str,
    pub icon: T,
}

pub fn Icon<'a, T: IconShape>(cx: Scope<'a, IconProps<'a, T>>) -> Element<'a> {
    let icon_style = format!(
        r#"
        width: 100%;
        max-width: {}px;
    "#,
        cx.props.width
    );

    cx.render(rsx! {
        svg {
            style: "{icon_style}",
            stroke: cx.props.stroke,
            stroke_width: cx.props.stroke_width,
            class: format_args!("{}", cx.props.class),
            height: format_args!("{}", cx.props.height),
            fill: format_args!("{}", cx.props.fill),
            view_box: format_args!("{}", cx.props.icon.view_box()),
            stroke_linecap: "round",
            stroke_linejoin: "round",
            cx.props.icon.child_elements()
        }
    })
}
