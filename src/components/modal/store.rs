use yew::{hook, use_context};
use crate::components::ModalHookResponse;

#[hook]
pub(crate) fn use_modal_store() -> ModalHookResponse {
  use_context::<ModalHookResponse>()
    .expect("use_modal_store must be used within ModalProvider")
}