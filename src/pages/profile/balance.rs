use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;

use crate::{
    components::atoms::{
        header_main::{HeaderCallOptions, HeaderEvent},
        Avatar, Header, Spinner,
    },
    hooks::use_client::use_client,
    pages::{profile::profile::Profile, route::Route},
};

#[inline_props]
pub fn Balance(cx: Scope) -> Element {
    let nav = use_navigator(cx);
    let client = use_client(cx);

    let original_profile = use_ref::<Profile>(cx, || Profile {
        displayname: String::from(""),
        avatar: None,
    });
    let is_loading_profile = use_ref::<bool>(cx, || true);

    use_coroutine(cx, |mut _rx: UnboundedReceiver<String>| {
        to_owned![client, original_profile, is_loading_profile];

        async move {
            let account_profile = client.get().account().get_profile().await.unwrap();

            let avatar_uri: Option<String> = match account_profile.avatar_url {
                Some(avatar) => {
                    let (server, id) = avatar.parts().unwrap();
                    let uri = format!("https://matrix-client.matrix.org/_matrix/media/r0/thumbnail/{}/{}?width=48&height=48&method=crop", server, id);
                    Some(String::from(uri))
                }
                None => None,
            };

            original_profile.set(Profile {
                displayname: account_profile.displayname.unwrap_or(String::from("")),
                avatar: avatar_uri,
            });

            let client = client.get();

            is_loading_profile.set(false);
        }
    });

    let displayname = original_profile.read().displayname.clone();
    let avatar = original_profile.read().avatar.clone();

    let header_event = move |evt: HeaderEvent| match evt.value {
        HeaderCallOptions::CLOSE => {
            nav.push(Route::ChatList {});
        }
        _ => {}
    };

    render! {
        rsx!(
            Header {
                text: "Tu Balance",
                on_event: header_event
            }
            if *is_loading_profile.read() {
                rsx!(
                    div {
                        class: "spinner-dual-ring--center",
                        Spinner {}
                    }
                )
            } else {
                rsx!(
                    // Profile info
                    div {
                        style: "
                            display: flex;
                            flex-direction: column;
                            align-items: center;
                            margin-top: 40px;
                        ",
                        section {
                            style: "
                                display: flex;
                                flex-direction: column;
                                gap: 4px;
                                align-items: center;
                            ",
                            Avatar {
                                name: displayname.clone(),
                                size: 80,
                                uri: avatar
                            }
                            p {
                                style: "
                                    color: var(--text-loud);
                                    font-family: Inter;
                                    font-size: 16px;
                                    font-style: normal;
                                    font-weight: 500;
                                    line-height: 24px;
                                ",
                                "{displayname}"
                            }
                            p {
                                style: "
                                    color: var(--text-subdued);
                                    font-family: Inter;
                                    font-size: 14px;
                                    font-style: normal;
                                    font-weight: 400;
                                    line-height: 20px;
                                ",
                                "0x00000"
                            }
                        }
                        // Balance general
                        section {
                            style: "
                                display: flex;
                                flex-direction: column;
                                gap: 4px;
                                justify-content: center;
                                margin-top: 40px;
                                align-items: center;
                            ",
                            h2 {
                                span {
                                    style: "
                                        color: var(--text-loud);
                                        font-family: Inter;
                                        font-size: 24px;
                                        font-style: normal;
                                        font-weight: 500;
                                        line-height: 90%;
                                    ",
                                    "$"
                                }
                                span {
                                    style: "
                                        color: var(--text-loud);
                                        font-family: Inter;
                                        font-size: 40px;
                                        font-style: normal;
                                        font-weight: 500;
                                        line-height: 90%;
                                        letter-spacing: -0.8px;
                                    ",
                                    "3,000.03"
                                }
                            }
                        }
                        // Token balances
                        section {
                            style: "
                                width: 100%;
                                margin-top: 40px;
                            ",
                            h4 {
                                style: "
                                    color: var(--text-normal);
                                    font-family: Inter;
                                    font-size: 16px;
                                    font-style: normal;
                                    font-weight: 500;
                                    line-height: 24px;
                                ",
                                "Token Balances"
                            }
                            article {
                                style: "
                                    border-radius: 16px;
                                    border: 1px solid var(--border-normal);
                                    background: var(--background-disabled);
                                    margin-top: 8px;
                                ",
                                div {
                                    style: "
                                        padding: 8px 12px;
                                        border-bottom: 1px solid var(--border-normal);
                                        display: flex;
                                        gap: 8px;
                                        justify-content: center;
                                        align-items: center;
                                    ",
                                    div {
                                        style: "
                                            height: 32px;
                                            min-width: 32px;
                                            background: blue;
                                        "
                                    }
                                    div {
                                        style: "
                                            display: flex;
                                            flex-direction: column;
                                            width: 100%;
                                        ",
                                        div {
                                            style: "
                                                display: flex;
                                                justify-content: space-between;
                                                align-items: flex-start;
                                            ",
                                            span {
                                                style: "
                                                    color: var(--text-muted);
                                                    font-family: Inter;
                                                    font-size: 16px;
                                                    font-style: normal;
                                                    font-weight: 500;
                                                    line-height: 24px;
                                                ",
                                                "KSM"
                                            }
                                            span {
                                                style: "
                                                    color: var(--text-normal);
                                                    font-family: Inter;
                                                    font-size: 16px;
                                                    font-style: normal;
                                                    font-weight: 500;
                                                    line-height: 24px;
                                                ",
                                                "$60"
                                            }
                                        }
                                        div {
                                            style: "
                                                display: flex;
                                                justify-content: space-between;
                                                align-items: flex-start;
                                            ",
                                            span {
                                                style: "
                                                    color: var(--text-normal);
                                                    font-family: Inter;
                                                    font-size: 12px;
                                                    font-style: normal;
                                                    font-weight: 500;
                                                    line-height: 16px;
                                                ",
                                                "3 KSM"
                                            }
                                            // badge
                                            div {
                                                style: "
                                                    display: flex;
                                                    padding: 2px 8px;
                                                    justify-content: center;
                                                    align-items: center;
                                                    gap: 4px;
                                                    border-radius: 96px;
                                                    background: var(--secondary-green-25);
                                                ",
                                                span {
                                                    style: "
                                                        color: var(--secondary-green-50);
                                                        text-align: center;
                                                        font-family: Inter;
                                                        font-size: 12px;
                                                        font-style: normal;
                                                        font-weight: 500;
                                                        line-height: 16px;
                                                    ",
                                                    "+$0.1"
                                                }
                                            }
                                        }

                                    }
                                }
                            }
                        }
                    }
                )
            }
        )
    }
}
