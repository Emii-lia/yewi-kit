use web_sys::MouseEvent;
use yew::{hook, Callback, Properties};
use yew_router::hooks::use_route;
use yew_router::prelude::use_navigator;
use crate::app::docs::routes::DocsRoute;
use crate::features::provider::SidebarContext;
use crate::features::sidebar::store::use_sidebar_store;

#[derive(PartialEq, Properties, Clone)]
pub struct HookParams {
  pub href: DocsRoute,
}

#[derive(PartialEq, Clone)]
pub struct HookResponse {
  pub is_active: bool,
  pub go: Callback<MouseEvent>,
}

#[hook]
pub(crate) fn use_nav_item(params: HookParams) -> HookResponse {
  let current_route = use_route::<DocsRoute>();
  let navigator = use_navigator().unwrap();
  let SidebarContext { toggle_open ,.. } = use_sidebar_store();
  let HookParams { href } = params;

  let go = {
    let href = href.clone();
    Callback::from(move |e: MouseEvent| {
      toggle_open.emit(e);
      navigator.push(&href);
    })
  };
  HookResponse { 
    is_active: matches!(current_route, Some(route) if route == href),
    go
  }
}

