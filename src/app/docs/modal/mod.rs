use yew::{component, html, Html};
use crate::previews::ModalPreview;

#[component(ModalPage)]
pub fn modal_page() -> Html {
  html! {
    <div class="ModalPage page-container">
      <ModalPreview/>
    </div>
  }
}