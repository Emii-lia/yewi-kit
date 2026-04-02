use yew::{component, html, Html};
use crate::previews::SelectPreview;

#[component(SelectPage)]
pub fn select_page() -> Html {
  html! {
    <div class="SelectPage page-container">
      <SelectPreview/>
    </div>
  }
}