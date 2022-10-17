use crate::fido_elements::*;
use dioxus::prelude::*;
use dioxus_router::Link;

struct NewsSrc<'a> {
    name: &'a str,
    room: &'a str,
    img_url: &'a str,
}

struct NewsSummary<'a> {
    title: &'a str,
    blob: &'a str,
    date: u32,
    img_url: Option<&'a str>,
}

const SOURCES: [NewsSrc; 1] = [NewsSrc {
    name: "Virto Updates",
    room: "#virto:virto.community",
    img_url: "https://matrix.virto.community/_matrix/media/r0/thumbnail/virto.community/ytBfVNEmJIMjpydIXjaXgCEa?width=85&height=85",
}];

const DUMMY_STORIES: [NewsSummary; 4] = [
    NewsSummary {
        title: "Fido everywhere!",
        date: 1665745453,
        img_url: Some("dummies/headline_devices.webp"),
        blob: "fido-everywhere",
    },
    NewsSummary {
        title: "Fido shell and its futuristic UI",
        date: 1665745453,
        img_url: Some("dummies/headline_pc.webp"),
        blob: "fido-ui",
    },
    NewsSummary {
        title: "Virto empowers local communities",
        date: 1665745453,
        img_url: None,
        blob: "virto-communities",
    },
    NewsSummary {
        title: "Secure payments with Virto",
        date: 1665745453,
        img_url: None,
        blob: "virto-payments",
    },
];

pub fn News(cx: Scope) -> Element {
    render! {
        nav {
            SOURCES.iter().map(|src| rsx! {
                frame { class: "box s r2", img { src: "{src.img_url}" } }
            })
            button { frame{ "+" } }
        }
        grid {
            class: "xxl",
            DUMMY_STORIES.iter().map(|NewsSummary{title, blob, date, img_url}| rsx!{
                frame {
                    class: "headline card xxl",
                    img_url.map(|url| rsx!(img { src: "{url}" })),
                    h2 { Link { to: "/news/{blob}", "{title}" } },
                    time { datetime: "{date}", "{date}" },
                }
            })
        }
    }
}
