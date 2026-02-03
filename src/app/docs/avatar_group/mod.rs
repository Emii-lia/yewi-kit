use yew::{function_component, html, Html};
use crate::previews::AvatarGroupPreview;

#[function_component(AvatarGroupPage)]
pub(crate) fn avatar_group_page() -> Html {
  html! {
    <div class="AvatarGroupPage page-container">
      <AvatarGroupPreview/>
    </div>
  }
}