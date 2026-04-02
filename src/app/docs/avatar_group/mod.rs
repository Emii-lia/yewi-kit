use yew::{component, html, Html};
use crate::previews::AvatarGroupPreview;

#[component(AvatarGroupPage)]
pub(crate) fn avatar_group_page() -> Html {
  html! {
    <div class="AvatarGroupPage page-container">
      <AvatarGroupPreview/>
    </div>
  }
}