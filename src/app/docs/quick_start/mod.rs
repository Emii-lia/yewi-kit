use yew::{function_component, html, Html};
use crate::features::QuickStart;

#[function_component(QuickStartPage)]
pub(crate) fn quick_start_page() -> Html {
  html! {
    <div class="QuickStartPage page-container">
      <QuickStart/>
    </div>
  }
}