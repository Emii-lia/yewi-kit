use yew::{html, Html};
use yew_router::{Routable, Switch};
use crate::app::docs::routes::DocsRoute;
use crate::app::docs::switch_docs;
use crate::app::ecosystem::route::EcosystemRoute;
use crate::app::ecosystem::switch_ecosystem;
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
  #[at("/ecosystem")]
  Ecosystem,
  #[at("/ecosystem/*")]
  EcosystemRoot,
  #[not_found]
  #[at("/404")]
  NotFound,
}

impl AppRoute {
    pub fn to_string(&self) -> String {
    match self {
      AppRoute::Home => "Installation".to_string(),
      AppRoute::NotFound => "Not Found".to_string(),
      AppRoute::DocsRoot | AppRoute::Docs => "Documentation".to_string(),
      AppRoute::EcosystemRoot | AppRoute::Ecosystem => "Ecosystem".to_string(),
    }
  }
}
pub fn switch_main(route: AppRoute) -> Html {
  match route {
    AppRoute::Home => html! {<Home/>},
    AppRoute::DocsRoot | AppRoute::Docs => html! {<Switch<DocsRoute> render={switch_docs}/>},
    AppRoute::EcosystemRoot | AppRoute::Ecosystem => html! {<Switch<EcosystemRoute> render={switch_ecosystem}/>},
    AppRoute::NotFound => html! {<NotFound/>},
  }
}