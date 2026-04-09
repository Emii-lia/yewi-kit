use yew::{hook, html, use_state, Callback, Classes, Html};

#[derive(Clone, Debug, PartialEq)]
pub struct OpenParams {
  pub children: Html,
  pub class: Option<Classes>,
}
#[derive(PartialEq, Clone, Debug)]
pub struct ModalHookResponse {
  pub onclose: Callback<()>,
  pub onopen: Callback<OpenParams>,
  pub is_open: bool,
  pub children: Html,
  pub classes: Classes,
}


#[hook]
pub(crate) fn use_modal() -> ModalHookResponse {
  let is_open = use_state(|| false);
  let children = use_state(|| html! {});
  let classes = use_state(|| Classes::from("Modal-content"));

  let onclose = {
    let is_open = is_open.clone();
    Callback::from(move |_| is_open.set(false))
  };
  let onopen = {
    let is_open = is_open.clone();
    let children = children.clone();
    let classes = classes.clone();
    Callback::from(move |params: OpenParams| {
      is_open.set(true);
      children.set(params.children);
      classes.set(params.class.unwrap_or_default());
    })
  };
  ModalHookResponse { onclose, onopen, is_open: *is_open, children: (*children).clone(), classes: (*classes).clone() }
}