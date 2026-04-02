use web_sys::MouseEvent;
use yew::{component, html, Callback, ContextProvider, Html};
use yew::html::ChildrenProps;
use crate::features::sidebar::hooks::{use_sidebar, RouteSection};

#[derive(Clone, PartialEq)]
pub struct SidebarContext {
  pub is_open: bool,
  pub toggle_open: Callback<MouseEvent>,
  pub route_group: Vec<RouteSection>
}
#[component(SidebarProvider)]
pub(crate) fn sidebar_provider(props: &ChildrenProps) -> Html {
  let sidebar_ctx = use_sidebar();
  html! {
    <ContextProvider<SidebarContext> context={sidebar_ctx}>
      {props.children.clone()}
    </ContextProvider<SidebarContext>>
  }
}