use web_sys::MouseEvent;
use yew::{hook, Callback};
use yew_icons::IconData;
use yew_router::prelude::{use_navigator, use_route};
use crate::app::docs::routes::DocsRoute;
use crate::components::sidebar::provider::SidebarContextType;
use crate::components::sidebar::store::use_sidebar_store;
use crate::types::components::ComponentNav;

#[hook]
pub fn use_yewi_sidebar() -> (
  (String, Vec<ComponentNav>),
  Vec<(String, Vec<ComponentNav>)>,
  Callback<(MouseEvent, DocsRoute), ()>,
  Callback<DocsRoute, bool>,
) {
  let navigator = use_navigator().unwrap();
  let current_route = use_route::<DocsRoute>().unwrap();

  let SidebarContextType {
    toggle_sidebar,
    is_mobile,
    ..
  } = use_sidebar_store();

  let is_active = {
    let current_route = current_route.clone();
    Callback::from(move |route: DocsRoute| {
      current_route == route
    })
  };

  let get_started: (String, Vec<ComponentNav>) = (
    "Get Started".to_string(),
    vec![
    ComponentNav::new(DocsRoute::Installation, Some(IconData::LUCIDE_CODE_2)),
    ComponentNav::new(DocsRoute::QuickStartPage, Some(IconData::LUCIDE_PLAY_CIRCLE)),
  ]);

  let components = ComponentNav::group_components();

  let on_navigate = {
    let navigator = navigator.clone();
    let toggle_sidebar = toggle_sidebar.clone();

    Callback::from(move |(e,route): (MouseEvent, DocsRoute)| {
      e.stop_propagation();
      navigator.push(&route);
      if is_mobile {
        toggle_sidebar.emit(());
      }
    })
  };

  (
    get_started,
    components,
    on_navigate,
    is_active
  )
}