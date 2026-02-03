use yew::{html, Html};
use crate::app::docs::avatar::AvatarPage;
use crate::app::docs::avatar_group::AvatarGroupPage;
use crate::app::docs::badge::BadgePage;
use crate::app::docs::button::ButtonPage;
use crate::app::docs::card::CardPage;
use crate::app::docs::carousel::CarouselPage;
use crate::app::docs::checkbox::CheckboxPage;
use crate::app::docs::collapse::CollapsePage;
use crate::app::docs::divider::DividerPage;
use crate::app::docs::dropdown::DropdownPage;
use crate::app::docs::file_input::FileInputPage;
use crate::app::docs::input::InputPage;
use crate::app::docs::installation::InstallationPage;
use crate::app::docs::modal::ModalPage;
use crate::app::docs::password_input::PasswordInputPage;
use crate::app::docs::progress::ProgressPage;
use crate::app::docs::radio::RadioPage;
use crate::app::docs::routes::DocsRoute;
use crate::app::docs::select::SelectPage;
use crate::app::docs::table::TablePage;
use crate::app::docs::tabs::TabsPage;
use crate::app::docs::textarea::TextareaPage;
use crate::app::docs::toast::ToastPage;
use crate::features::Sidebar;

mod button;
mod badge;
mod avatar;
mod input;
mod select;
mod checkbox;
mod modal;
mod textarea;
mod radio;
mod tabs;
mod avatar_group;
mod collapse;
mod file_input;
mod divider;
mod card;
mod password_input;
mod progress;
mod carousel;
mod dropdown;
mod toast;
mod table;
mod installation;
pub mod routes;

pub fn switch_docs(route: DocsRoute) -> Html {
  let current_route = match route {
    DocsRoute::Docs => html! {<InstallationPage/>},
    DocsRoute::Installation => html! {<InstallationPage/>},
    DocsRoute::AvatarPage => html! {<AvatarPage/>},
    DocsRoute::AvatarGroupPage => html! {<AvatarGroupPage/>},
    DocsRoute::BadgePage => html! {<BadgePage/>},
    DocsRoute::ButtonPage => html! {<ButtonPage/>},
    DocsRoute::CardPage => html! {<CardPage/>},
    DocsRoute::CarouselPage => html! {<CarouselPage/>},
    DocsRoute::CheckboxPage => html! {<CheckboxPage/>},
    DocsRoute::CollapsePage => html! {<CollapsePage/>},
    DocsRoute::DividerPage => html! {<DividerPage/>},
    DocsRoute::DropdownPage => html! {<DropdownPage/>},
    DocsRoute::FileInput => html! {<FileInputPage/>},
    DocsRoute::InputPage => html! {<InputPage/>},
    DocsRoute::ModalPage => html! {<ModalPage/>},
    DocsRoute::PasswordInputPage => html! {<PasswordInputPage/>},
    DocsRoute::ProgressPage => html! {<ProgressPage/>},
    DocsRoute::RadioPage => html! {<RadioPage/>},
    DocsRoute::SelectPage => html! {<SelectPage/>},
    DocsRoute::TablePage => html! {<TablePage/>},
    DocsRoute::TabsPage => html! {<TabsPage/>},
    DocsRoute::TextareaPage => html! {<TextareaPage/>},
    DocsRoute::ToastPage => html! {<ToastPage/>},
  };

  html! {
    <div class="docs-app">
      <Sidebar/>
      {current_route}
    </div>
  }
}