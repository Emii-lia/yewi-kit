use yew::{html, Html};
use yew_router::{Routable, Switch};
use crate::app::{
  AvatarGroupPage,
  AvatarPage,
  BadgePage,
  ButtonPage,
  CheckboxPage,
  CollapsePage,
  DividerPage,
  FileInputPage,
  Home,
  InputPage,
  ModalPage,
  NotFound,
  RadioPage,
  SelectPage,
  TabsPage,
  TextareaPage
};
use crate::app::card::CardPage;
use crate::app::carousel::CarouselPage;
use crate::app::dropdown::DropdownPage;
use crate::app::installation::InstallationPage;
use crate::app::password_input::PasswordInputPage;
use crate::app::progress::ProgressPage;
use crate::app::table::TablePage;
use crate::app::toast::ToastPage;
use crate::features::Sidebar;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
  #[at("/")]
  Home,
  #[at("/docs")]
  Docs,
  #[at("/docs/*")]
  DocsRoot,
  #[not_found]
  #[at("/404")]
  NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum DocsRoute {
  #[at("/docs")]
  Docs,
  #[at("/docs/installation")]
  Installation,
  #[at("/docs/avatar")]
  AvatarPage,
  #[at("/docs/avatar-group")]
  AvatarGroupPage,
  #[at("/docs/badge")]
  BadgePage,
  #[at("/docs/button")]
  ButtonPage,
  #[at("/docs/card")]
  CardPage,
  #[at("/docs/carousel")]
  CarouselPage,
  #[at("/docs/checkbox")]
  CheckboxPage,
  #[at("/docs/collapse")]
  CollapsePage,
  #[at("/docs/divider")]
  DividerPage,
  #[at("/docs/dropdown")]
  DropdownPage,
  #[at("/docs/file-input")]
  FileInput,
  #[at("/docs/input")]
  InputPage,
  #[at("/docs/modal")]
  ModalPage,
  #[at("/docs/password-input")]
  PasswordInputPage,
  #[at("/docs/progress")]
  ProgressPage,
  #[at("/docs/radio")]
  RadioPage,
  #[at("/docs/select")]
  SelectPage,
  #[at("/docs/table")]
  TablePage,
  #[at("/docs/tabs")]
  TabsPage,
  #[at("/docs/textarea")]
  TextareaPage,
  #[at("/docs/toast")]
  ToastPage,
}

impl AppRoute {
  pub(crate) fn iter() -> impl Iterator<Item = AppRoute> {
    vec![
      AppRoute::Home,
    ].into_iter()
  }

  pub fn to_string(&self) -> String {
    match self {
      AppRoute::Home => "Installation".to_string(),
      AppRoute::NotFound => "Not Found".to_string(),
      AppRoute::Docs => "Documentation".to_string(),
      AppRoute::DocsRoot => "Documentation".to_string()
    }
  }
}

impl DocsRoute {
  pub fn iter() -> impl Iterator<Item = DocsRoute> {
    vec![
      DocsRoute::Installation,
      DocsRoute::AvatarPage,
      DocsRoute::AvatarGroupPage,
      DocsRoute::BadgePage,
      DocsRoute::ButtonPage,
      DocsRoute::CardPage,
      DocsRoute::CarouselPage,
      DocsRoute::CheckboxPage,
      DocsRoute::CollapsePage,
      DocsRoute::DividerPage,
      DocsRoute::DropdownPage,
      DocsRoute::FileInput,
      DocsRoute::InputPage,
      DocsRoute::ModalPage,
      DocsRoute::PasswordInputPage,
      DocsRoute::ProgressPage,
      DocsRoute::RadioPage,
      DocsRoute::SelectPage,
      DocsRoute::TablePage,
      DocsRoute::TabsPage,
      DocsRoute::TextareaPage,
      DocsRoute::ToastPage
    ].into_iter()
  }

  pub fn to_string(&self) -> String {
    match self {
      DocsRoute::Docs => "Documentation".to_string(),
      DocsRoute::Installation => "Installation".to_string(),
      DocsRoute::AvatarPage => "Avatar".to_string(),
      DocsRoute::AvatarGroupPage => "Avatar Group".to_string(),
      DocsRoute::BadgePage => "Badge".to_string(),
      DocsRoute::ButtonPage => "Button".to_string(),
      DocsRoute::CardPage => "Card".to_string(),
      DocsRoute::CarouselPage => "Carousel".to_string(),
      DocsRoute::CheckboxPage => "Checkbox".to_string(),
      DocsRoute::CollapsePage => "Collapse".to_string(),
      DocsRoute::DividerPage => "Divider".to_string(),
      DocsRoute::DropdownPage => "Dropdown".to_string(),
      DocsRoute::FileInput => "File Input".to_string(),
      DocsRoute::InputPage => "Input".to_string(),
      DocsRoute::PasswordInputPage => "Password Input".to_string(),
      DocsRoute::ProgressPage => "Progress".to_string(),
      DocsRoute::ModalPage => "Modal".to_string(),
      DocsRoute::RadioPage => "Radio".to_string(),
      DocsRoute::SelectPage => "Select".to_string(),
      DocsRoute::TablePage => "Table".to_string(),
      DocsRoute::TabsPage => "Tabs".to_string(),
      DocsRoute::TextareaPage => "Textarea".to_string(),
      DocsRoute::ToastPage => "Toast".to_string(),
    }
  }
}

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
pub fn switch(route: AppRoute) -> Html {
  match route {
    AppRoute::Home => html! {<Home/>},
    AppRoute::DocsRoot | AppRoute::Docs => html! {<Switch<DocsRoute> render={switch_docs}/>},
    AppRoute::NotFound => html! {<NotFound/>},
  }
}