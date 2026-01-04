use yew::{hook, use_state, Callback};

#[derive(Clone, PartialEq)]
pub struct HookResponse {
  pub is_visible: bool,
  pub toggle_visible: Callback<()>
}
#[hook]
pub(crate) fn use_password_input() -> HookResponse {
  let is_visible = use_state(|| false);
  
  let toggle_visible = {
    let is_visible = is_visible.clone();
    Callback::from(move |_| is_visible.set(!*is_visible))
  };
  
  HookResponse {
    is_visible: *is_visible,
    toggle_visible
  }
}