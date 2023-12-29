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
    let variant = match cx.props.variant {
        Variant::Round => {
            r#"
						border-radius: 100%;
					"#
        }
        Variant::SemiRound => {
            r#"
						border-radius: 20%;
					"#
        }
    };

    let avatar_style = r#"
        width: var(--avatar-size);
        min-width: var(--avatar-size);
        height: var(--avatar-size);
        background: linear-gradient(var(--accent-aqua-25), var(--accent-aqua-50));
        display: flex;
        align-items: center;
        justify-content: center;
    "#;
    let avatar_style = format!("{}\n{}\n{}", size_avatar, avatar_style, variant);

    let initial_style = r#"
        font-size: calc(var(--avatar-size) * 0.4);
        color: var(--text-normal);
    "#;
    cx.render(rsx! {
      match &cx.props.uri {
          Some(uri)=> {
            rsx!(
              img{
                style: "{avatar_style}",
                src: "{uri}"
              }
            )
          },
          None=>{
            let initial: Vec<char> = cx.props.name.chars().collect();
            let initial = initial[0].to_uppercase();

            rsx!(
              div{
                style: "{avatar_style}",
                span{
                  style: "{initial_style}",
                  "{initial}"
                }
              }
            )
          }
        }
    })
}
