use yew::{function_component, html, Html};
use crate::previews::SelectPreview;

#[function_component(SelectPage)]
pub fn select_page() -> Html {
  html! {
    <div class="SelectPage page-container">
      <SelectPreview/>
    </div>
  }
}