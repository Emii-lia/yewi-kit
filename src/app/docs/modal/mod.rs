use yew::{function_component, html, Html};
use crate::previews::ModalPreview;

#[function_component(ModalPage)]
pub fn modal_page() -> Html {
  html! {
    <div class="ModalPage page-container">
      <ModalPreview/>
    </div>
  }
}