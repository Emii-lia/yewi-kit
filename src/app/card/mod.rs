use yew::{function_component, html, Html};
use crate::previews::CardPreview;

#[function_component(CardPage)]
pub(crate) fn card_page() -> Html {
  html! {
    <div class="CardPage page-container">
      <CardPreview/>
    </div>
  }
}