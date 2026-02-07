use yew::hook;
use crate::app::docs::routes::DocsRoute;

#[derive(Clone)]
pub struct RouteSection {
  pub title: String,
  pub routes: Vec<DocsRoute>
}
#[hook]
pub fn use_sidebar() -> Vec<RouteSection> {
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

  route_sections
}