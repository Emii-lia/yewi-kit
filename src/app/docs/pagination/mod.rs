use yew::{function_component, html, Html};
use crate::previews::PaginationPreview;

#[function_component(PaginationPage)]
pub(crate) fn pagination_page() -> Html {
  html! {
    <div class="PaginationPage page-container">
      <PaginationPreview/>
    </div>
  }
}