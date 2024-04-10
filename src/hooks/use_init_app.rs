use std::collections::HashMap;

use crate::components::atoms::message::Messages;
use crate::services::matrix::matrix::TimelineThread;
use crate::{
    components::molecules::{input_message::ReplyingTo, rooms::CurrentRoom},
    pages::login::LoggedIn,
    MatrixClientState,
};
use dioxus::prelude::*;
use matrix_sdk::encryption::verification::SasVerification;
use ruma::api::client::uiaa::AuthType;

use super::use_auth::CacheLogin;
use super::use_notification::NotificationItem;
use super::use_public::PublicState;
use super::use_room_preview::PreviewRoom;
use super::use_rooms::RoomsList;
use super::use_send_attach::SendAttachStatus;
use super::use_session::UserSession;
use super::{use_attach::AttachFile, use_modal::ModalState};

#[derive(Clone)]
pub enum BeforeSession {
    Login,
    Signup,
    Guest,
}

#[derive(Clone, Debug)]
pub struct MessageDispatchId {
    pub value: HashMap<String, Option<String>>,
}

pub fn use_init_app() {
    use_context_provider::<Signal<LoggedIn>>(|| Signal::new(LoggedIn(false)));
    use_context_provider::<Signal<MatrixClientState>>(|| {
        Signal::new(MatrixClientState { client: None })
    });
    use_context_provider::<Signal<ModalState>>(|| {
        Signal::new(ModalState {
            show: false,
            account: None,
        })
    });

    // Temporarily moved here because Route has an unexpected
    // change when we push a ChatRoom from a different nest route

    use_context_provider::<Signal<CurrentRoom>>(|| Signal::new(CurrentRoom::default()));
    use_context_provider::<Signal<PreviewRoom>>(|| Signal::new(PreviewRoom::default()));
    use_context_provider::<Signal<RoomsList>>(|| Signal::new(RoomsList::default()));
    use_context_provider::<Signal<Messages>>(|| Signal::new(Vec::new()));
    use_context_provider::<Signal<Option<AttachFile>>>(|| Signal::new(None));
    use_context_provider::<Signal<Option<ReplyingTo>>>(|| Signal::new(None));
    use_context_provider::<Signal<NotificationItem>>(|| Signal::new(NotificationItem::default()));

    use_context_provider::<Signal<Option<SasVerification>>>(|| Signal::new(None));
    use_context_provider::<Signal<Option<TimelineThread>>>(|| Signal::new(None));

    use_context_provider::<Signal<BeforeSession>>(|| Signal::new(BeforeSession::Guest));
    use_context_provider::<Signal<Option<CacheLogin>>>(|| Signal::new(None));
    use_context_provider::<Signal<Vec<AuthType>>>(|| Signal::new(vec![]));

    use_context_provider::<Signal<Option<UserSession>>>(|| Signal::new(None));

    use_context_provider::<Signal<MessageDispatchId>>(|| {
        Signal::new(MessageDispatchId {
            value: HashMap::new(),
        })
    });
    use_context_provider::<Signal<SendAttachStatus>>(|| Signal::new(SendAttachStatus::Loading(0)));
    use_context_provider::<Signal<PublicState>>(|| Signal::new(PublicState::default()));
}
