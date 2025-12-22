use yew::{html, Html};
use yew_router::Routable;
use crate::app::{BadgePage, ButtonPage, Home};
use crate::app::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
  #[at("/")]
  Home,
  #[at("/button")]
  ButtonPage,
  #[at("/badge")]
  BadgePage,
  #[not_found]
  #[at("/404")]
  NotFound,
}

pub fn switch(route: AppRoute) -> Html {
  match route {
    AppRoute::Home => html! {<Home/>},
    AppRoute::ButtonPage => html! {<ButtonPage/>},
    AppRoute::BadgePage => html! {<BadgePage/>},
    AppRoute::NotFound => html! {<NotFound/>},
  }
}