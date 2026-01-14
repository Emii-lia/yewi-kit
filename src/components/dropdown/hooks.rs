use gloo::events::EventListener;
use web_sys::{window, Event, HtmlElement};
use web_sys::wasm_bindgen::JsCast;
use yew::{hook, use_effect_with, use_node_ref, use_state, NodeRef};
use crate::components::dropdown::use_dropdown_store;

pub(crate) struct HookResponse {
  pub(crate) dropdown_ref: NodeRef,
}
#[hook]
pub(crate) fn use_dropdown() -> HookResponse {
  let dropdown_ref = use_node_ref();
  let ctx = use_dropdown_store();
  let listener = use_state(|| Option::<EventListener>::None);

  {
    let dropdown_ref = dropdown_ref.clone();
    let listener = listener.clone();
    let close = ctx.close.clone();
    let is_open = ctx.is_open;

    use_effect_with(
      is_open,
      move |_| {
        if !is_open {
          listener.set(None);
          return;
        }

        let dropdown_node = dropdown_ref.cast::<HtmlElement>();

        if let Some(node) = dropdown_node {
          let close_cb = close.clone();
          let event_listener = EventListener::new(&window().unwrap(), "click", move |event: &Event| {
            if let Some(target) = event.target() {
              let target_element = target.dyn_into::<HtmlElement>().ok();
              if let Some(target_el) = target_element {
                if !node.contains(Some(&target_el)) {
                  close_cb.emit(());
                }
              }
            }
          });

          listener.set(Some(event_listener));
        }
      }
    );
  }

  HookResponse {
    dropdown_ref,
  }
}