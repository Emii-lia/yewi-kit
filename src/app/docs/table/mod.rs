use yew::{component, html, Html};
use crate::previews::TablePreview;

#[component(TablePage)]
pub(crate) fn table_page() -> Html {
  html! {
    <div class="TablePage page-container">
      <TablePreview/>
    </div>
  }
}