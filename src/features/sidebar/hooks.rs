use web_sys::MouseEvent;
use yew::{hook, use_state, Callback};
use crate::app::docs::routes::DocsRoute;
use crate::features::provider::SidebarContext;

#[derive(Clone, PartialEq)]
pub struct RouteSection {
  pub title: String,
  pub routes: Vec<DocsRoute>
}
#[hook]
pub fn use_sidebar() -> SidebarContext {
  let is_open = use_state(|| false);
  let routes: Vec<DocsRoute> = DocsRoute::iter()
    .filter(|route| *route != DocsRoute::Docs && *route != DocsRoute::NotFound)
    .map(|route| route.clone())
    .collect();

  let mut route_sections: Vec<RouteSection> = Vec::new();
  let mut get_started: Vec<DocsRoute> = Vec::new();
  let mut components: Vec<DocsRoute> = Vec::new();

  for route in routes {
    match route {
      DocsRoute::Installation => get_started.push(route),
      DocsRoute::QuickStartPage => get_started.push(route),
      _ => components.push(route)
    }
  }

  route_sections.push(RouteSection {
    title: "Getting Started".to_string(),
    routes: get_started
  });
  route_sections.push(RouteSection {
    title: "Components".to_string(),
    routes: components
  });

  let toggle_open = {
    let is_open = is_open.clone();
    Callback::from(move |_| {
      is_open.set(!*is_open);
    })
  };

  SidebarContext { route_group: route_sections, is_open: *is_open, toggle_open }
}