use yew::{classes, component, html, Callback, Classes, ContextProvider, Html, Properties};
use crate::components::sidebar::hooks::{use_sidebar_provider, SidebarProviderHookParams};
use crate::components::sidebar::types::{SidebarConfig, SidebarState};

#[derive(Clone, Debug, PartialEq)]
pub struct SidebarContextType {
  pub state: SidebarState,
  pub open: bool,
  pub set_open: Callback<bool>,
  pub open_mobile: bool,
  pub set_open_mobile: Callback<bool>,
  pub is_mobile: bool,
  pub toggle_sidebar: Callback<()>,
}

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
  pub children: Html,
  #[prop_or(true)]
  pub default_open: bool,
  #[prop_or_default]
  pub open: Option<bool>,
  #[prop_or_default]
  pub onopenchange: Option<Callback<bool>>,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub style: String,
}

#[component(SidebarProvider)]
pub fn sidebar_provider(props: &Props) -> Html {
  let ctx = use_sidebar_provider(SidebarProviderHookParams {
    default_open: props.default_open,
    open: props.open,
    onopenchange: props.onopenchange.clone(),
  });
  let SidebarConfig {
    width,
    width_icon,
    ..
  } = SidebarConfig::default();

  html! {
    <ContextProvider<SidebarContextType> context={ctx}>
      <div
        class={classes!("SidebarProvider", &props.class)}
        style={format!(
          "--sidebar-width: {}; --sidebar-width-icon: {}; {}",
          width,
          width_icon,
          &props.style
        )}
      >
        {props.children.clone()}
      </div>
    </ContextProvider<SidebarContextType>>
  }
}