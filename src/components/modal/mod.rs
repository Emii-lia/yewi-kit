mod hooks;
mod provider;
mod store;

use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback, Html};
use crate::components::Button;
use crate::types::{ButtonVariant, Size};
pub(crate) use hooks::*;
pub use provider::*;
pub use store::*;

#[function_component(Modal)]
pub(crate) fn modal() -> Html {
  let modal_vars = use_modal_store();
  let ModalHookResponse {
    on_close,
    classes,
    is_open,
    children,
    ..
  } = modal_vars;
  
  html! {
    if is_open {
      <div
        class={classes!("Modal")}
        onclick={{
          let on_close = on_close.clone();
          move |_| { on_close.emit(()); }
        }}
      >
        <div
          class={classes!("modal-body", classes)}
          onclick={|e: MouseEvent| { e.stop_propagation();}}
        >
          <div class="modal-header">
            <Button
              variant={ButtonVariant::Tertiary}
              size={Size::Small}
              onclick={{
                let on_close = on_close.clone();
                Callback::from(move |e: MouseEvent| {
                  e.stop_propagation();
                  on_close.emit(());
                })
              }}
            >
              {"x"}
            </Button>
          </div>
          {children.clone()}
        </div>
      </div>
    }
  }
}