mod hooks;

use yew::{classes, function_component, html, Html, Properties};
use yew_router::prelude::Link;
use crate::app::DocsRoute;
use crate::features::sidebar::components::nav_item::hooks::{use_nav_item, HookParams, HookResponse};

#[derive(Properties, PartialEq ,Clone)]
pub struct Props {
  pub label: String,
  pub href: DocsRoute,
}

#[function_component(NavItem)]
pub(crate) fn nav_item(props: &Props) -> Html {
  let HookResponse { is_active } = use_nav_item(HookParams { href: props.href.clone() });
  
  html! {
    <Link<DocsRoute>
      to={props.href.clone()}
      classes={classes!("NavItem", is_active.then_some("nav-active"))}
      
    >
      {props.label.clone()}
    </Link<DocsRoute>>
  }
}