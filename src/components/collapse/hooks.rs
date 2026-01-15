use yew::{hook, use_node_ref, use_state, Callback, NodeRef};

#[derive(Clone, PartialEq)]
pub(crate) struct HookResponse {
  pub(crate) is_open: bool,
  pub(crate) toggle: Callback<()>,
  pub(crate) checkbox_ref: NodeRef,
}

#[hook]
pub(crate) fn use_collapse() -> HookResponse {
  let is_open = use_state(|| false);
  let checkbox_ref = use_node_ref();

  let toggle = {
    let is_open = is_open.clone();
    let checkbox_ref = checkbox_ref.clone();
    Callback::from(move |_| {
      let checkbox_node = checkbox_ref.cast::<web_sys::HtmlInputElement>();
      if let Some(checkbox) = checkbox_node {
        checkbox.set_checked(!*is_open);
      }
      is_open.set(!*is_open);
    })
  };

  HookResponse {
    is_open: *is_open,
    toggle,
    checkbox_ref,
  }
}