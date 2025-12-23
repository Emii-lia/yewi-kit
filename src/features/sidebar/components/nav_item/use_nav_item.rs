use yew::{hook, Properties};
use yew_router::hooks::use_route;
use crate::app::AppRoute;

#[derive(PartialEq, Properties, Clone)]
pub struct HookParams {
  pub href: AppRoute,
}

#[derive(PartialEq, Clone)]
pub struct HookResponse {
  pub is_active: bool
}

#[hook]
pub(crate) fn use_nav_item(params: HookParams) -> HookResponse {
  let current_route = use_route::<AppRoute>();
  let HookParams { href } = params;
  
  HookResponse { is_active: matches!(current_route, Some(route) if route == href) }
}

