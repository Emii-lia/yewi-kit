use yew::{classes, component, html, Callback, Classes, Html, Properties};
use yew_icons::IconData;
use crate::components::sidebar::provider::SidebarContextType;
use store::use_sidebar_store;
use crate::components::button::{Button, ButtonVariant};
use crate::components::sidebar::types::{SidebarConfig, SidebarPosition};
use crate::types::Size;

pub mod provider;
pub mod types;
pub mod hooks;
pub mod store;

#[derive(Properties, Debug, PartialEq)]
pub struct Props {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(SidebarPosition::Left)]
  pub position: SidebarPosition
}
#[component(Sidebar)]
pub fn sidebar(props: &Props) -> Html {
  let SidebarContextType {
    is_mobile,
    state,
    open_mobile,
    set_open_mobile,
    ..
  } = use_sidebar_store();
  let SidebarConfig {
    width_mobile,
    width,
    width_icon
  } = SidebarConfig::default();

  if is_mobile {
    html! {
      <div
        class={classes!("Sidebar", &props.class)}
        data-open-mobile={open_mobile.to_string()}
        data-position={props.position.clone().as_str()}
        data-mobile={"true"}
        style={format!("--sidebar-width: {};", width_mobile)}
      >
        {props.children.clone()}
      </div>
    }
  } else {
    html! {
      <div
        class={classes!("Sidebar", &props.class)}
        data-state={state.clone().as_str()}
        data-position={props.position.clone().as_str()}
        data-mobile={"false"}
        style={format!(
          "--sidebar-width: {}; --sidebar-width-icon: {};",
          width,
          width_icon,
        )}
      >
        {props.children.clone()}
      </div>
    }
  }
}

#[derive(Properties, Debug, PartialEq)]
pub struct SidebarTriggerProps {
  #[prop_or(Size::Small)]
  pub size: Size,
  #[prop_or(IconData::LUCIDE_SIDEBAR)]
  pub icon: IconData,
  #[prop_or_default]
  pub onclick: Callback<()>,
  #[prop_or_default]
  pub class: Classes
}

#[component(SidebarTrigger)]
pub fn sidebar_trigger(props: &SidebarTriggerProps) -> Html {
  let SidebarContextType {
    toggle_sidebar,
    ..
  } = use_sidebar_store();

  html! {
    <Button
      size={props.size.clone()}
      icon={props.icon.clone()}
      onclick={{
        let toggle_sidebar = toggle_sidebar.clone();
        let onclick = props.onclick.clone();
        Callback::from(move |_| {
          toggle_sidebar.emit(());
          onclick.emit(());
        })
      }}
      variant={ButtonVariant::Tertiary}
      class={classes!("SidebarTrigger", &props.class)}
      title={"Toggle Sidebar"}
    />
  }
}
