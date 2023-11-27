use dioxus::prelude::*;
#[derive(PartialEq, Props)]
pub struct AvatarProps {
    name: String,
    size: u8,
    #[props(!optional)]
    uri: Option<String>,
}

pub fn Avatar(cx: Scope<AvatarProps>) -> Element {
    let size_avatar = format!("--avatar-size: {}px;", cx.props.size);
    let avatar_style = r#"
        width: var(--avatar-size);
        min-width: var(--avatar-size);
        height: var(--avatar-size);
        background: linear-gradient(var(--accent-aqua-25), var(--accent-aqua-50));
        border-radius: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    "#;
    let avatar_style = format!("{}\n{}", size_avatar, avatar_style);

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
