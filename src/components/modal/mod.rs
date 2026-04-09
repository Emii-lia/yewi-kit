mod hooks;
mod provider;
mod store;

use web_sys::MouseEvent;
use yew::{classes, component, html, Callback, Html};
use yew_icons::IconData;
use crate::components::button::{Button, ButtonVariant};
use crate::types::{Size};
pub(crate) use hooks::*;
pub(crate) use provider::*;
pub(crate) use store::*;

#[component(Modal)]
pub(crate) fn modal() -> Html {
  let modal_vars = use_modal_store();
  let ModalHookResponse {
    onclose,
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
          let onclose = onclose.clone();
          move |_| { onclose.emit(()); }
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
                let onclose = onclose.clone();
                Callback::from(move |e: MouseEvent| {
                  e.stop_propagation();
                  onclose.emit(());
                })
              }}
              icon={IconData::LUCIDE_X}
            />
          </div>
          {children.clone()}
        </div>
      </div>
    }
  }
}