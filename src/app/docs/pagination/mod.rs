use yew::{component, html, Html};
use crate::previews::PaginationPreview;

#[component(PaginationPage)]
pub(crate) fn pagination_page() -> Html {
  html! {
    <div class="PaginationPage page-container">
      <PaginationPreview/>
    </div>
  }
}