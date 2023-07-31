use dioxus::prelude::*;
#[derive(Props)]
pub struct AvatarProps<'a> {
    name: &'a str,
    size: u8,
    #[props(!optional)]
    uri: Option<&'a String>,
}

pub fn Avatar<'a>(cx: Scope<'a, AvatarProps<'a>>) -> Element<'a> {
    let size_avatar = format!("--avatar-size: {}px;", cx.props.size);
    let avatar_style = r#"
        width: var(--avatar-size);
        min-width: var(--avatar-size);
        height: var(--avatar-size);
        background: var(--surface-2);
        border-radius: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
    "#;
    let avatar_style = format!("{}\n{}", size_avatar, avatar_style);

    
    let initial_style = r#"
        font-size: calc(var(--avatar-size) * 0.8)px;
        color: var(--text-1);
    "#;
    cx.render(rsx! {
      match cx.props.uri {
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
