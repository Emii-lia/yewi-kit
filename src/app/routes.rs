use yew::{html, Html};
use yew_router::Routable;
use crate::app::{BadgePage, ButtonPage, Home};
use crate::app::avatar::AvatarPage;
use crate::app::checkbox::CheckboxPage;
use crate::app::input::InputPage;
use crate::app::modal::ModalPage;
use crate::app::not_found::NotFound;
use crate::app::radio::RadioPage;
use crate::app::select::SelectPage;
use crate::app::tabs::TabsPage;
use crate::app::textarea::TextareaPage;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
  #[at("/")]
  Home,
  #[at("/avatar")]
  AvatarPage,
  #[at("/badge")]
  BadgePage,
  #[at("/button")]
  ButtonPage,
  #[at("/checkbox")]
  CheckboxPage,
  #[at("/input")]
  InputPage,
  #[at("/modal")]
  ModalPage,
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
      AppRoute::BadgePage,
      AppRoute::ButtonPage,
      AppRoute::CheckboxPage,
      AppRoute::InputPage,
      AppRoute::ModalPage,
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
    AppRoute::BadgePage => html! {<BadgePage/>},
    AppRoute::ButtonPage => html! {<ButtonPage/>},
    AppRoute::CheckboxPage => html! {<CheckboxPage/>},
    AppRoute::InputPage => html! {<InputPage/>},
    AppRoute::ModalPage => html! {<ModalPage/>},
    AppRoute::RadioPage => html! {<RadioPage/>},
    AppRoute::SelectPage => html! {<SelectPage/>},
    AppRoute::TabsPage => html! {<TabsPage/>},
    AppRoute::TextareaPage => html! {<TextareaPage/>},
    AppRoute::NotFound => html! {<NotFound/>},
  }
}