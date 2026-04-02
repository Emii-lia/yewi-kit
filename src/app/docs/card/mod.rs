use yew::{component, html, Html};
use crate::previews::CardPreview;

#[component(CardPage)]
pub(crate) fn card_page() -> Html {
  html! {
    <div class="CardPage page-container">
      <CardPreview/>
    </div>
  }
}