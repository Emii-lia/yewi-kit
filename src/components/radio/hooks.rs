use web_sys::{Event, EventTarget, HtmlInputElement};
use web_sys::wasm_bindgen::JsCast;
use yew::{hook, Callback, Properties, TargetCast};

#[derive(Properties, Clone, PartialEq)]
pub struct HookParams {
  pub onchange: Callback<bool>,
}

#[derive(Clone, PartialEq)]
pub struct HookResponse {
  pub on_change: Callback<Event>,
}

#[hook]
pub fn use_radio(props: HookParams) -> HookResponse {
  let HookParams { onchange } = props.clone();

  let on_change = {
    let onchange = onchange.clone();
    Callback::from(move |e: Event| {
      let target: Option<EventTarget> = e.target();
      let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
      if let Some(input) = input {
        onchange.emit(input.checked());
      }
    })
  };

  HookResponse {
    on_change,
  }
}