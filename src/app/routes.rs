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
use crate::app::textarea::TextareaPage;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
  #[at("/")]
  Home,
  #[at("/button")]
  ButtonPage,
  #[at("/badge")]
  BadgePage,
  #[at("/avatar")]
  AvatarPage,
  #[at("/input")]
  InputPage,
  #[at("/select")]
  SelectPage,
  #[at("/checkbox")]
  CheckboxPage,
  #[at("/modal")]
  ModalPage,
  #[at("/textarea")]
  TextareaPage,
  #[at("/radio")]
  RadioPage,
  #[not_found]
  #[at("/404")]
  NotFound,
}

impl AppRoute {
  pub(crate) fn iter() -> impl Iterator<Item = AppRoute> {
    vec![
      AppRoute::Home,
      AppRoute::ButtonPage,
      AppRoute::BadgePage,
      AppRoute::AvatarPage,
      AppRoute::InputPage,
      AppRoute::SelectPage,
      AppRoute::CheckboxPage,
      AppRoute::ModalPage,
      AppRoute::TextareaPage,
      AppRoute::RadioPage,
    ].into_iter()
  }
}

pub fn switch(route: AppRoute) -> Html {
  match route {
    AppRoute::Home => html! {<Home/>},
    AppRoute::ButtonPage => html! {<ButtonPage/>},
    AppRoute::BadgePage => html! {<BadgePage/>},
    AppRoute::AvatarPage => html! {<AvatarPage/>},
    AppRoute::InputPage => html! {<InputPage/>},
    AppRoute::SelectPage => html! {<SelectPage/>},
    AppRoute::CheckboxPage => html! {<CheckboxPage/>},
    AppRoute::ModalPage => html! {<ModalPage/>},
    AppRoute::TextareaPage => html! {<TextareaPage/>},
    AppRoute::RadioPage => html! {<RadioPage/>},
    AppRoute::NotFound => html! {<NotFound/>},
  }
}