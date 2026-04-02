use yew::{component, html, Html};
use crate::features::QuickStart;

#[component(QuickStartPage)]
pub(crate) fn quick_start_page() -> Html {
  html! {
    <div class="QuickStartPage page-container">
      <QuickStart/>
    </div>
  }
}