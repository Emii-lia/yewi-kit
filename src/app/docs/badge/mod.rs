use yew::{component, html, Html};
use crate::previews::BadgePreview;

#[component(BadgePage)]
pub(crate) fn badge_page() -> Html {
  html! {
    <div class="BadgePage page-container">
      <BadgePreview/>
    </div>
  }
}