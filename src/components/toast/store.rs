use yew::{hook, use_context};
use crate::components::toast::hooks::ToastContext;

#[hook]
pub(crate) fn use_toast_store() -> ToastContext {
  use_context::<ToastContext>()
    .expect("ToastContext not found. Make sure you are using ToastProvider.")
}