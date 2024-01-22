use dioxus::prelude::*;

#[derive(PartialEq)]
pub enum Variant {
    Round,
    SemiRound,
}

#[derive(PartialEq, Props)]
pub struct AvatarProps {
    name: String,
    size: u8,
    #[props(!optional)]
    uri: Option<String>,
    #[props(default = Variant::Round)]
    variant: Variant,
}

pub fn Avatar(cx: Scope<AvatarProps>) -> Element {
    let size_avatar = format!("--avatar-size: {}px;", cx.props.size);
    let avatar_style = format!("{}", size_avatar);

    let variant = match cx.props.variant {
        Variant::Round => "avatar--round",
        Variant::SemiRound => "avatar--semi-round",
    };

    cx.render(rsx! {
      match &cx.props.uri {
          Some(uri)=> rsx!(
            img {
              class: "avatar {variant}",
              style: "{avatar_style}",
              src: "{uri}"
            }
          ),
          None=>{
            let initial: Vec<char> = cx.props.name.chars().collect();
            let initial = initial[0].to_uppercase();

            rsx!(
              div{
                class: "avatar {variant}",
                style: "{avatar_style}",
                span{
                  class: "avatar--initial",
                  "{initial}"
                }
              }
            )
          }
        }
    })
}
