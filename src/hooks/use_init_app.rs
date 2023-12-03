use crate::components::atoms::message::Messages;
use crate::pages::chat::chat::{NotificationHandle, NotificationType};
use crate::services::matrix::matrix::TimelineThread;
use crate::{
    components::molecules::{input_message::ReplyingTo, rooms::CurrentRoom},
    pages::{
        chat::chat::{ListHeight, NotificationItem},
        login::LoggedIn,
    },
    MatrixClientState,
};
use dioxus::prelude::*;
use matrix_sdk::encryption::verification::{Emoji, SasVerification};
use ruma::api::client::uiaa::AuthType;

use super::{use_attach::AttachFile, use_modal::ModalState};

pub enum BeforeSession {
    Login,
    Signup,
}

#[allow(clippy::needless_return)]
pub fn use_init_app(cx: &ScopeState) {
    use_shared_state_provider::<LoggedIn>(cx, || LoggedIn(false));
    use_shared_state_provider::<MatrixClientState>(cx, || MatrixClientState { client: None });
    use_shared_state_provider::<ModalState>(cx, || ModalState {
        show: false,
        account: None,
    });

    // Temporarily moved here because Route has an unexpected
    // change when we push a ChatRoom from a different nest route

    use_shared_state_provider::<CurrentRoom>(cx, || CurrentRoom {
        id: String::from(""),
        name: String::from(""),
        avatar_uri: None,
    });
    use_shared_state_provider::<Messages>(cx, || Vec::new());
    use_shared_state_provider::<Option<AttachFile>>(cx, || None);
    use_shared_state_provider::<Option<ReplyingTo>>(cx, || None);
    use_shared_state_provider::<ListHeight>(cx, || ListHeight {
        height: { format!("height: calc(100vh - 72px - {}px );", 82) },
    });

    use_shared_state_provider::<NotificationItem>(cx, || NotificationItem {
        title: String::from(""),
        body: String::from(""),
        show: false,
        handle: NotificationHandle {
            value: NotificationType::None,
        },
    });

    use_shared_state_provider::<Option<SasVerification>>(cx, || None);
    use_shared_state_provider::<Option<TimelineThread>>(cx, || None);

    use_shared_state_provider::<BeforeSession>(cx, || BeforeSession::Signup);
    use_shared_state_provider::<Vec<AuthType>>(cx, || vec![]);
}
