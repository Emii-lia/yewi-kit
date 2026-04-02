mod hooks;

use yew::{classes, component, html, Html, Properties};
use yew_router::Routable;
use crate::app::docs::routes::DocsRoute;
use crate::features::sidebar::components::nav_item::hooks::{use_nav_item, HookParams, HookResponse};

#[derive(Properties, PartialEq ,Clone)]
pub struct Props {
  pub label: String,
  pub href: DocsRoute,
}

#[component(NavItem)]
pub(crate) fn nav_item(props: &Props) -> Html {
  let HookResponse { is_active, go } = use_nav_item(HookParams { href: props.href.clone() });
  
  html! {
    <a
      class={classes!("NavItem", is_active.then_some("nav-active"))}
      href={props.href.to_path()}
      onclick={go}
    >
      {props.label.clone()}
    </a>
  }
}