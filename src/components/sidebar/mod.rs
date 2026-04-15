use web_sys::MouseEvent;
use yew::{classes, component, html, Callback, Classes, Html, Properties};
use yew_icons::{Icon, IconData};
use crate::components::sidebar::provider::SidebarContextType;
use store::use_sidebar_store;
use crate::components::button::{Button, ButtonVariant};
use crate::components::collapse::{Collapse, CollapseContent, CollapseIndicator, CollapseTrigger, CollapseVariant};
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
    sidebar_ref,
    ..
  } = use_sidebar_store();
  let SidebarConfig {
    width_mobile,
    width,
    width_icon
  } = SidebarConfig::default();

  if is_mobile {
    html! {
      <>
        <div
          class={classes!("Sidebar", &props.class)}
          data-open-mobile={open_mobile.to_string()}
          data-position={props.position.clone().as_str()}
          data-mobile={"true"}
          ref={sidebar_ref.clone()}
          style={format!("--sidebar-width: {};", width_mobile)}
        >
          {props.children.clone()}
        </div>
        <div
          class={"sidebar-overlay"}
          onclick={{
            let set_open_mobile = set_open_mobile.clone();
            Callback::from(move |_| {
              if open_mobile {
                set_open_mobile.emit(false);
              }
            })
          }}
        />
      </>
    }
  } else {
    html! {
      <div
        class={classes!("Sidebar", &props.class)}
        data-el="sidebar"
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
      id={"sidebar-trigger"}
      onclick={{
        let toggle_sidebar = toggle_sidebar.clone();
        let onclick = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
          e.stop_propagation();
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

#[derive(Properties, Debug, PartialEq)]
pub struct SidebarChildrenWithClassWithIconProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub icon: Option<IconData>,
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
pub fn sidebar_group_title(props: &SidebarChildrenWithClassWithIconProps) -> Html {
  html! {
    <div class={classes!("SidebarGroupTitle", &props.class)}>
      {html! {
        if let Some(icon) = props.icon.clone() {
          <Icon data={icon} class={"sidebar-group-title-icon"}/>
        }
      }}
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
  pub action: Html,
  #[prop_or_default]
  pub icon: Option<IconData>,
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
      <div class={"sidebar-menu-item-content"}>
        {html! {
          if let Some(icon) = props.icon.clone() {
            <Icon data={icon} class={"sidebar-menu-item-icon"}/>
          }
        }}
        {props.children.clone()}
      </div>
      <div class={"SidebarMenuItemAction"}>
        {props.action.clone()}
      </div>
    </div>
  }
}

#[component(SidebarSubMenu)]
pub fn sidebar_sub_menu(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <Collapse
      variant={CollapseVariant::Toggle}
      class={classes!("SidebarSubMenu", &props.class)}
    >
      {props.children.clone()}
    </Collapse>
  }
}

#[component(SidebarSubMenuTitle)]
pub fn sidebar_sub_menu_title(props: &SidebarChildrenWithClassWithIconProps) -> Html {
  html! {
    <CollapseTrigger
      class={classes!("SidebarSubMenuTitle", &props.class)}
      indicator={CollapseIndicator::Chevron}
      icon={props.icon.clone()}
    >
      {props.children.clone()}
    </CollapseTrigger>
  }
}

#[component(SidebarSubMenuContent)]
pub fn sidebar_sub_menu_content(props: &SidebarChildrenWithClassProps) -> Html {
  html! {
    <CollapseContent class={classes!("SidebarSubMenuContent", &props.class)}>
      {props.children.clone()}
    </CollapseContent>
  }
}