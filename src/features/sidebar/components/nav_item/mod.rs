mod use_nav_item;

use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::Link;
use crate::app::AppRoute;
use crate::features::sidebar::components::nav_item::use_nav_item::{use_nav_item, HookParams, HookResponse};

#[derive(Properties, PartialEq ,Clone)]
pub struct Props {
  pub label: String,
  pub href: AppRoute,
}

#[function_component(NavItem)]
pub(crate) fn nav_item(props: &Props) -> Html {
  let HookResponse { is_active } = use_nav_item(HookParams { href: props.href.clone() });
  
  html! {
    <Link<AppRoute>
      to={props.href.clone()}
      classes={classes!("NavItem", is_active.then_some("nav-active"))}
      
    >
      {props.label.clone()}
    </Link<AppRoute>>
  }
}