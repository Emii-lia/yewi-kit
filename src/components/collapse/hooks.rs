use yew::{hook, use_node_ref, use_state, Callback, NodeRef};
use crate::components::collapse::CollapseVariant;

#[derive(Clone, PartialEq)]
pub(crate) struct HookResponse {
  pub(crate) is_open: bool,
  pub(crate) toggle: Callback<()>,
  pub(crate) checkbox_ref: NodeRef,
}

#[derive(Clone, PartialEq)]
pub(crate) struct HookParams {
  pub(crate) variant: CollapseVariant,
}

#[hook]
pub(crate) fn use_collapse(params: HookParams) -> HookResponse {
  let HookParams { variant } = params;

  let is_open = use_state(|| match variant {
    CollapseVariant::Focus => false,
    CollapseVariant::Toggle(default_open) => default_open,
  });

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