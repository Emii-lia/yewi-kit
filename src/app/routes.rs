use yew::{html, Html};
use yew_router::Routable;
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
use crate::app::password_input::PasswordInputPage;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
  #[at("/")]
  Home,
  #[at("/avatar")]
  AvatarPage,
  #[at("/avatar-group")]
  AvatarGroupPage,
  #[at("/badge")]
  BadgePage,
  #[at("/button")]
  ButtonPage,
  #[at("/card")]
  CardPage,
  #[at("/checkbox")]
  CheckboxPage,
  #[at("/collapse")]
  CollapsePage,
  #[at("/divider")]
  DividerPage,
  #[at("/file-input")]
  FileInput,
  #[at("/input")]
  InputPage,
  #[at("/modal")]
  ModalPage,
  #[at("/password-input")]
  PasswordInputPage,
  #[at("/radio")]
  RadioPage,
  #[at("/select")]
  SelectPage,
  #[at("/tabs")]
  TabsPage,
  #[at("/textarea")]
  TextareaPage,
  #[not_found]
  #[at("/404")]
  NotFound,
}

impl AppRoute {
  pub(crate) fn iter() -> impl Iterator<Item = AppRoute> {
    vec![
      AppRoute::Home,
      AppRoute::AvatarPage,
      AppRoute::AvatarGroupPage,
      AppRoute::BadgePage,
      AppRoute::ButtonPage,
      AppRoute::CardPage,
      AppRoute::CheckboxPage,
      AppRoute::CollapsePage,
      AppRoute::DividerPage,
      AppRoute::FileInput,
      AppRoute::InputPage,
      AppRoute::ModalPage,
      AppRoute::PasswordInputPage,
      AppRoute::RadioPage,
      AppRoute::SelectPage,
      AppRoute::TabsPage,
      AppRoute::TextareaPage,
    ].into_iter()
  }
}

pub fn switch(route: AppRoute) -> Html {
  match route {
    AppRoute::Home => html! {<Home/>},
    AppRoute::AvatarPage => html! {<AvatarPage/>},
    AppRoute::AvatarGroupPage => html! {<AvatarGroupPage/>},
    AppRoute::BadgePage => html! {<BadgePage/>},
    AppRoute::ButtonPage => html! {<ButtonPage/>},
    AppRoute::CardPage => html! {<CardPage/>},
    AppRoute::CheckboxPage => html! {<CheckboxPage/>},
    AppRoute::CollapsePage => html! {<CollapsePage/>},
    AppRoute::DividerPage => html! {<DividerPage/>},
    AppRoute::FileInput => html! {<FileInputPage/>},
    AppRoute::InputPage => html! {<InputPage/>},
    AppRoute::ModalPage => html! {<ModalPage/>},
    AppRoute::PasswordInputPage => html! {<PasswordInputPage/>},
    AppRoute::RadioPage => html! {<RadioPage/>},
    AppRoute::SelectPage => html! {<SelectPage/>},
    AppRoute::TabsPage => html! {<TabsPage/>},
    AppRoute::TextareaPage => html! {<TextareaPage/>},
    AppRoute::NotFound => html! {<NotFound/>},
  }
}