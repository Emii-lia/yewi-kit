use yew::{html, Html};
use yew_router::Routable;
use crate::app::{BadgePage, ButtonPage, Home};
use crate::app::avatar::AvatarPage;
use crate::app::input::InputPage;
use crate::app::not_found::NotFound;

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
      AppRoute::InputPage
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
    AppRoute::NotFound => html! {<NotFound/>},
  }
}