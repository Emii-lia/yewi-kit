use web_sys::MouseEvent;
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
pub struct SidebarProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(SidebarPosition::Left)]
  pub position: SidebarPosition
}
#[component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
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

#[derive(Properties, Debug, PartialEq)]
pub struct SidebarChildrenWithClassProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
}
#[component(SidebarHeader)]
pub fn sidebar_header(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <div class={classes!("SidebarHeader", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[component(SidebarTitle)]
pub fn sidebar_title(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <h2 class={classes!("SidebarTitle", &props.class)}>
      {props.children.clone()}
    </h2>
  }
}

#[component(SidebarFooter)]
pub fn sidebar_footer(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <div class={classes!("SidebarFooter", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[component(SidebarContent)]
pub fn sidebar_content(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <div class={classes!("SidebarContent", "no-scrollbar", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[component(SidebarGroup)]
pub fn sidebar_group(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <div class={classes!("SidebarGroup", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[component(SidebarGroupTitle)]
pub fn sidebar_group_title(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <div class={classes!("SidebarGroupTitle", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[component(SidebarGroupContent)]
pub fn sidebar_group_content(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <div class={classes!("SidebarGroupContent", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[component(SidebarMenu)]
pub fn sidebar_menu(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <div class={classes!("SidebarMenu", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[derive(Properties, Debug, PartialEq)]
pub struct SidebarMenuItemProps {
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub children: Html,
  #[prop_or_default]
  pub active: bool,
  #[prop_or_default]
  pub onclick: Callback<()>,
  #[prop_or_default]
  pub action: Html
}
#[component(SidebarMenuItem)]
pub fn sidebar_menu_item(props: &SidebarMenuItemProps) -> Html {
  html! {
    <div 
      class={classes!(
        "SidebarMenuItem", 
        &props.class,
      )}
      data-active={props.active.to_string()}
      onclick={{
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
          e.stop_propagation();
          onclick.emit(());
        })
      }}
    >
      {props.children.clone()}
      <div class={"SidebarMenuItemAction"}>
        {props.action.clone()}
      </div>
    </div>
  }
}