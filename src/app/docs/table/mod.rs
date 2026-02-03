use yew::{function_component, html, Html};
use crate::previews::TablePreview;

#[function_component(TablePage)]
pub(crate) fn table_page() -> Html {
  html! {
    <div class="TablePage page-container">
      <TablePreview/>
    </div>
  }
}