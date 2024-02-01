use crate::{
    components::molecules::input_message::ReplyingTo,
    hooks::use_session::{use_session, UseSessionState},
    services::matrix::matrix::{
        EventOrigin, RoomMember, TimelineMessage, TimelineMessageReply, TimelineMessageType,
        TimelineRelation, TimelineThread,
    },
};

use dioxus::prelude::*;

#[allow(clippy::needless_return)]

pub fn use_text_message_factory(cx: &ScopeState) -> impl MessageFactory<String> {
    let session = use_session(cx).clone();
    TextMessageFactory { session }
}

pub fn use_image_message_factory(cx: &ScopeState) -> impl MessageFactory<String> {
    let session = use_session(cx).clone();
    ImageMessageFactory { session }
}

pub fn use_reply_message_factory(cx: &ScopeState) -> impl MessageFactory<ReplyingTo> {
    let session = use_session(cx).clone();
    ReplyMessageFactory { session }
}

pub fn use_custom_thread_message_factory(cx: &ScopeState) -> impl MessageFactory<TimelineThread> {
    let session = use_session(cx).clone();
    CustomThreadMessageFactory { session }
}

pub enum MessageFactoryType {
    Text,
    Reply,
    CustomThread,
}

pub trait MessageFactory<T> {
    fn create_message(
        &self,
        content: &TimelineMessageType,
        uuid: &str,
        time: &str,
        related: &T,
    ) -> TimelineRelation;
}

pub struct TextMessageFactory {
    session: UseSessionState,
}

impl MessageFactory<String> for TextMessageFactory {
    fn create_message(
        &self,
        content: &TimelineMessageType,
        uuid: &str,
        time: &str,
        r: &String,
    ) -> TimelineRelation {
        TimelineRelation::None(TimelineMessage {
            body: content.clone(),
            event_id: Some(uuid.to_string()),
            sender: RoomMember {
                id: match self.session.get() {
                    Some(u) => u.user_id,
                    None => String::from(""),
                },
                name: String::from(""),
                avatar_uri: None,
            },
            origin: EventOrigin::ME,
            time: time.to_string(),
        })
    }
}

pub struct ImageMessageFactory {
    session: UseSessionState,
}

impl MessageFactory<String> for ImageMessageFactory {
    fn create_message(
        &self,
        content: &TimelineMessageType,
        uuid: &str,
        time: &str,
        r: &String,
    ) -> TimelineRelation {
        TimelineRelation::None(TimelineMessage {
            body: content.clone(),
            event_id: Some(uuid.to_string()),
            sender: RoomMember {
                id: match self.session.get() {
                    Some(u) => u.user_id,
                    None => String::from(""),
                },
                name: String::from(""),
                avatar_uri: None,
            },
            origin: EventOrigin::ME,
            time: time.to_string(),
        })
    }
}

struct ReplyMessageFactory {
    session: UseSessionState,
}

impl MessageFactory<ReplyingTo> for ReplyMessageFactory {
    fn create_message(
        &self,
        content: &TimelineMessageType,
        uuid: &str,
        time: &str,
        r: &ReplyingTo,
    ) -> TimelineRelation {
        TimelineRelation::Reply(TimelineMessageReply {
            event: TimelineMessage {
                body: content.clone(),
                event_id: Some(uuid.to_string()),
                sender: RoomMember {
                    id: match self.session.get() {
                        Some(u) => u.user_id,
                        None => String::from(""),
                    },
                    name: String::from(""),
                    avatar_uri: None,
                },
                origin: EventOrigin::ME,
                time: time.to_string(),
            },
            reply: Some(TimelineMessage {
                event_id: Some(r.event_id.clone()),
                sender: RoomMember {
                    id: String::from(""),
                    name: r.display_name.clone(),
                    avatar_uri: r.avatar_uri.clone(),
                },
                body: r.content.clone(),
                origin: r.origin.clone(),
                time: String::from(""),
            }),
        })
    }
}

struct CustomThreadMessageFactory {
    session: UseSessionState,
}

impl MessageFactory<TimelineThread> for CustomThreadMessageFactory {
    fn create_message(
        &self,
        content: &TimelineMessageType,
        uuid: &str,
        time: &str,
        t: &TimelineThread,
    ) -> TimelineRelation {
        let mut t = t.clone();

        t.thread.push(TimelineMessage {
            body: content.clone(),
            event_id: Some(uuid.to_string()),
            sender: RoomMember {
                id: match self.session.get() {
                    Some(u) => u.user_id,
                    None => String::from(""),
                },
                name: String::from(""),
                avatar_uri: None,
            },
            origin: EventOrigin::ME,
            time: time.to_string(),
        });

        TimelineRelation::CustomThread(t.clone())
    }
}
