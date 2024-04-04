use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct CommunityProps {
    class: Option<String>,
    title: String,
    icon: String,
    background: String,
}

pub fn Community(props: CommunityProps) -> Element {
    let content__background = format!("community__content--{}", props.background);
    let class_content = props.class.unwrap_or("".to_owned());

    rsx!(
        section { class: "community {class_content}",
            div { class: "community__content {content__background}",
                div { class: "community__icon", "{props.icon}" }
            }
            span { class: "community__title", "{props.title}" }
        }
    )
}
