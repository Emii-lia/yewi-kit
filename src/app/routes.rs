use yew::{html, Html};
use yew_router::{Routable, Switch};
use crate::app::docs::routes::DocsRoute;
use crate::app::docs::switch_docs;
use crate::app::not_found::NotFound;
use crate::app::page::Home;

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
pub fn switch_main(route: AppRoute) -> Html {
  match route {
    AppRoute::Home => html! {<Home/>},
    AppRoute::DocsRoot | AppRoute::Docs => html! {<Switch<DocsRoute> render={switch_docs}/>},
    AppRoute::NotFound => html! {<NotFound/>},
  }
}