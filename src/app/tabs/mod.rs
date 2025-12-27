use yew::{function_component, html, Html};
use crate::previews::TabsPreview;

#[function_component(TabsPage)]
pub(crate) fn tabs_page() -> Html {
  html! {
    <div class="TabsPage page-container">
      <TabsPreview/>
    </div>
  }
}