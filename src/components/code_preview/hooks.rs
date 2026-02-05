use web_sys::{window, MouseEvent};
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::prelude::Closure;
use yew::{hook, html, use_state, AttrValue, Callback, Html};

#[derive(Clone, PartialEq)]
pub(crate) struct HookParams {
  pub code: AttrValue
}

pub(crate) struct HookResponse {
  pub copied: bool,
  pub on_copy: Callback<MouseEvent>
}

#[hook]
pub(crate) fn use_code_preview(params: HookParams) -> HookResponse {
  let copied = use_state(|| false);

  let on_copy = {
    let code = params.code.clone();
    let copied = copied.clone();
    Callback::from(move |_| {
      if let Some(window) = window() {
        let _ = window.navigator().clipboard().write_text(&code);
        copied.set(true);
        let copied = copied.clone();
        let closure = Closure::wrap(Box::new(move || {
          copied.set(false);
        }) as Box<dyn Fn()>);
        window
          .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 2000)
          .expect("should register timeout");
        closure.forget();
      }
    })
  };

  HookResponse {
    copied: *copied,
    on_copy,
  }
}