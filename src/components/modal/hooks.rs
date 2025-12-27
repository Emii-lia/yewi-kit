use yew::{hook, html, use_state, Callback, Classes, Html};

#[derive(Clone, Debug, PartialEq)]
pub struct OpenParams {
  pub children: Html,
  pub class: Option<Classes>,
}
#[derive(PartialEq, Clone, Debug)]
pub struct ModalHookResponse {
  pub on_close: Callback<()>,
  pub on_open: Callback<OpenParams>,
  pub is_open: bool,
  pub children: Html,
  pub classes: Classes,
}


#[hook]
pub(crate) fn use_modal() -> ModalHookResponse {
  let is_open = use_state(|| false);
  let children = use_state(|| html! {});
  let classes = use_state(|| Classes::from("Modal-content"));

  let on_close = {
    let is_open = is_open.clone();
    Callback::from(move |_| is_open.set(false))
  };
  let on_open = {
    let is_open = is_open.clone();
    let children = children.clone();
    let classes = classes.clone();
    Callback::from(move |params: OpenParams| {
      is_open.set(true);
      children.set(params.children);
      classes.set(params.class.unwrap_or_default());
    })
  };
  ModalHookResponse { on_close, on_open, is_open: *is_open, children: (*children).clone(), classes: (*classes).clone() }
}