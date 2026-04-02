use yew::{component, html, Html};
use crate::previews::TabsPreview;

#[component(TabsPage)]
pub(crate) fn tabs_page() -> Html {
  html! {
    <div class="TabsPage page-container">
      <TabsPreview/>
    </div>
  }
}