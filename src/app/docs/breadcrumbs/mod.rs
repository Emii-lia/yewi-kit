use yew::{function_component, html, Html};
use crate::previews::BreadcrumbsPreview;

#[function_component(BreadcrumbsPage)]
pub(crate) fn breadcrumbs_page() -> Html {
  html! {
    <div class="BreadcrumbsPage page-container">
      <BreadcrumbsPreview/>
    </div>
  }
}