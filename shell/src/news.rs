use crate::fido_elements::*;
use dioxus::prelude::*;

struct NewsSrc<'a> {
    name: &'a str,
    room: &'a str,
    img_url: &'a str,
}

struct NewsSummary<'a> {
    title: &'a str,
    date: u32,
}

const SOURCES: [NewsSrc; 1] = [NewsSrc {
    name: "Virto Updates",
    room: "#virto:virto.community",
    img_url: "https://matrix.virto.community/_matrix/media/r0/thumbnail/virto.community/ytBfVNEmJIMjpydIXjaXgCEa?width=85&height=85",
}];

const DUMMY_STORIES: [NewsSummary; 4] = [
    NewsSummary {
        title: "Mini Fidos selling like hot bread!",
        date: 1665745453,
    },
    NewsSummary {
        title: "Fido shell and its futuristic UI",
        date: 1665745453,
    },
    NewsSummary {
        title: "Virto empowers local communities",
        date: 1665745453,
    },
    NewsSummary {
        title: "Secure payments with Virto",
        date: 1665745453,
    },
];

pub fn News(cx: Scope) -> Element {
    render! {
        grid {
            DUMMY_STORIES.iter().map(|NewsSummary{title, date}| rsx!(
                frame {
                    class: "headline",
                    h2 { "{title}" },
                    time { datetime: "{date}" },
                }
            ))
        }
    }
}
