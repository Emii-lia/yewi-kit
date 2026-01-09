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
  pub highlight_code: Box<dyn Fn(&str) -> Html>,
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

  fn highlight_code(code: &str) -> Html {
    let mut out: Vec<Html> = Vec::new();
    let mut buf = String::new();


    let mut in_tag = false;
    let mut in_attr_value = false;


    for ch in code.chars() {
      match ch {
        '<' => {
          if !buf.is_empty() {
            out.push(html! { <span>{ buf.clone() }</span> });
            buf.clear();
          }
          in_tag = true;
          buf.push(ch);
        }
        '>' => {
          buf.push(ch);
          out.push(html! { <span class="tag">{ buf.clone() }</span> });
          buf.clear();
          in_tag = false;
          in_attr_value = false;
        }
        '"' if in_tag => {
          buf.push(ch);
          if in_attr_value {
            out.push(html! { <span class="attr-value">{ buf.clone() }</span> });
            buf.clear();
          }
          in_attr_value = !in_attr_value;
        }
        '=' if in_tag && !in_attr_value => {
          if !buf.is_empty() {
            out.push(html! { <span class="attr">{ buf.clone() }</span> });
            buf.clear();
          }
          out.push(html! { <span class="punct">{"="}</span> });
        }
        ' ' if in_tag && !in_attr_value => {
          if !buf.is_empty() {
            out.push(html! { <span class="attr">{ buf.clone() }</span> });
            buf.clear();
          }
          out.push(html! { " " });
        }
        _ => buf.push(ch),
      }
    }


    if !buf.is_empty() {
      out.push(html! { <span>{ buf }</span> });
    }


    html! { <> { out } </> }
  }

  HookResponse {
    copied: *copied,
    highlight_code: Box::new(highlight_code),
    on_copy,
  }
}