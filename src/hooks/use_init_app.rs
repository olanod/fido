use dioxus::prelude::*;

use crate::{pages::login::LoggedIn, MatrixClientState};

use super::use_modal::ModalState;

#[allow(clippy::needless_return)]
pub fn use_init_app(cx: &ScopeState) {
    use_shared_state_provider::<LoggedIn>(cx, || LoggedIn {
        is_logged_in: false,
    });
    use_shared_state_provider::<MatrixClientState>(cx, || MatrixClientState { client: None });
    use_shared_state_provider::<ModalState>(cx, || ModalState {
        show: false,
        account: None,
    });
}
