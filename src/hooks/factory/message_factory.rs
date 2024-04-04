use crate::{
    components::molecules::input_message::ReplyingTo,
    hooks::use_session::UserSession,
    services::matrix::matrix::{
        EventOrigin, RoomMember, TimelineMessage, TimelineMessageReply, TimelineMessageType,
        TimelineRelation, TimelineThread,
    },
};

#[allow(clippy::needless_return)]

pub fn use_message_factory() -> MessageFactoryType {
    MessageFactoryType {}
}

#[derive(Clone)]
pub struct MessageFactoryType {}

impl MessageFactoryType {
    pub fn text(&self) -> impl MessageFactory {
        TextMessageFactory {}
    }

    pub fn reply(&self, relation: ReplyingTo) -> impl MessageFactory {
        ReplyMessageFactory { relation }
    }

    pub fn thread(&self, relation: TimelineThread) -> impl MessageFactory {
        CustomThreadMessageFactory { relation }
    }
}

pub trait MessageFactory {
    fn create_message(
        &self,
        content: &TimelineMessageType,
        uuid: &str,
        time: &str,
        session: &UserSession,
    ) -> TimelineRelation;
}

pub struct TextMessageFactory {}

impl MessageFactory for TextMessageFactory {
    fn create_message(
        &self,
        content: &TimelineMessageType,
        uuid: &str,
        time: &str,
        session: &UserSession,
    ) -> TimelineRelation {
        TimelineRelation::None(TimelineMessage {
            body: content.clone(),
            event_id: uuid.to_string(),
            sender: RoomMember {
                id: session.user_id.clone(),
                name: String::from("x"),
                avatar_uri: None,
            },
            origin: EventOrigin::ME,
            time: time.to_string(),
        })
    }
}

struct ReplyMessageFactory {
    relation: ReplyingTo,
}

impl MessageFactory for ReplyMessageFactory {
    fn create_message(
        &self,
        content: &TimelineMessageType,
        uuid: &str,
        time: &str,
        session: &UserSession,
    ) -> TimelineRelation {
        TimelineRelation::Reply(TimelineMessageReply {
            event: TimelineMessage {
                body: content.clone(),
                event_id: uuid.to_string(),
                sender: RoomMember {
                    id: session.user_id.clone(),
                    name: String::from("x"),
                    avatar_uri: None,
                },
                origin: EventOrigin::ME,
                time: time.to_string(),
            },
            reply: Some(TimelineMessage {
                event_id: self.relation.event_id.clone(),
                sender: RoomMember {
                    id: String::from(""),
                    name: self.relation.display_name.clone(),
                    avatar_uri: self.relation.avatar_uri.clone(),
                },
                body: self.relation.content.clone(),
                origin: self.relation.origin.clone(),
                time: String::from(""),
            }),
        })
    }
}

struct CustomThreadMessageFactory {
    relation: TimelineThread,
}

impl MessageFactory for CustomThreadMessageFactory {
    fn create_message(
        &self,
        content: &TimelineMessageType,
        uuid: &str,
        time: &str,
        session: &UserSession,
    ) -> TimelineRelation {
        let mut t = self.relation.clone();

        t.thread.push(TimelineMessage {
            body: content.clone(),
            event_id: uuid.to_string(),
            sender: RoomMember {
                id: session.user_id.clone(),
                name: String::from("x"),
                avatar_uri: None,
            },
            origin: EventOrigin::ME,
            time: time.to_string(),
        });

        TimelineRelation::CustomThread(t.clone())
    }
}
