use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum Variant {
    Round,
    SemiRound,
}

#[derive(PartialEq, Props, Clone)]
pub struct AvatarProps {
    name: String,
    size: u8,
    #[props(!optional)]
    uri: Option<String>,
    #[props(default = Variant::Round)]
    variant: Variant,
}

pub fn Avatar(props: AvatarProps) -> Element {
    let size_avatar = format!("--avatar-size: {}px;", props.size);
    let avatar_style = format!("{}", size_avatar);

    let variant = match props.variant {
        Variant::Round => "avatar--round",
        Variant::SemiRound => "avatar--semi-round",
    };

    rsx! {
        match &props.uri {
            Some(uri) => rsx!(
                img {
                    class: "avatar {variant}",
                    style: "{avatar_style}",
                    src: "{uri}"
                }
            ),
            None => {
                let initial: Vec<char> = props.name.chars().collect();
                let initial = initial[0].to_uppercase();
        
                rsx!(
                    div{
                        class: "avatar {variant}",
                        style: "{avatar_style}",
                        span{ class: "avatar--initial", "{initial}" }
                    }
                )
            }
        }
    }
}
