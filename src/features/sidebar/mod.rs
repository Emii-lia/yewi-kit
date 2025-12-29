mod components {
  pub mod nav_item;
}

use yew::{function_component, html, Html};
use crate::app::AppRoute;
use crate::features::sidebar::components::nav_item::NavItem;

#[function_component(Sidebar)]
pub(crate) fn sidebar() -> Html {
  html! {
    <div class="Sidebar">
      <div class="sidebar-header">
        <h1 class="sidebar-title">{"Yewi-kit"}</h1>
      </div>
      <div class="sidebar-menu">
        {
          AppRoute::iter().filter(|route| *route != AppRoute::NotFound).map(|route| {
            let label = match route {
              AppRoute::Home => "Home",
              AppRoute::AvatarPage => "Avatar",
              AppRoute::AvatarGroupPage => "Avatar Group",
              AppRoute::BadgePage => "Badge",
              AppRoute::ButtonPage => "Button",
              AppRoute::CheckboxPage => "Checkbox",
              AppRoute::CollapsePage => "Collapse",
              AppRoute::ModalPage => "Modal",
              AppRoute::InputPage => "Input",
              AppRoute::RadioPage => "Radio",
              AppRoute::SelectPage => "Select",
              AppRoute::TabsPage => "Tabs",
              AppRoute::TextareaPage => "Textarea",
              AppRoute::NotFound => "Not Found",
            }.to_string();
            html! {
              <NavItem label={label} href={route.clone()}/>
            }
          }).collect::<Html>()

        }

      </div>
    </div>
  }
}