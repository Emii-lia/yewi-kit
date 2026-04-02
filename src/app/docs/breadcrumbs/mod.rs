use yew::{component, html, Html};
use crate::previews::BreadcrumbsPreview;

#[component(BreadcrumbsPage)]
pub(crate) fn breadcrumbs_page() -> Html {
  html! {
    <div class="BreadcrumbsPage page-container">
      <BreadcrumbsPreview/>
    </div>
  }
}