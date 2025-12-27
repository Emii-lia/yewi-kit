use yew::{hook, use_state, Hook, UseStateHandle};
use yew::virtual_dom::VChild;
use crate::components::tabs::{Tab, TabsProps};

pub(crate) struct HookResponse {
  pub active: UseStateHandle<String>,
  pub tabs: Vec<VChild<Tab>>,
}
#[hook]
pub(crate) fn use_tabs(props: &TabsProps) -> HookResponse {
  let active = use_state(|| {
    props
      .active_tab
      .clone()
      .unwrap_or_else(|| {
        props
          .children
          .iter()
          .next()
          .map(|tab| tab.props.value.clone())
          .unwrap_or_default()
      })
  });

  let tabs = props.children.iter().collect::<Vec<_>>();

  HookResponse {
    active,
    tabs
  }
}