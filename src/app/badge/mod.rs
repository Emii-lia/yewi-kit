use yew::{function_component, html, Html};
use crate::previews::BadgePreview;

#[function_component(BadgePage)]
pub(crate) fn badge_page() -> Html {
  html! {
    <div class="BadgePage page-container">
      <BadgePreview/>
    </div>
  }
}