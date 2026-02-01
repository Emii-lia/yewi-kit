mod types;
mod provider;
mod store;
mod hooks;

use web_sys::MouseEvent;
use yew::{classes, function_component, html, Callback, Children, Classes, Html, Properties};
pub(crate) use types::*;
pub(crate) use store::*;
use crate::components::dropdown::hooks::{use_dropdown, HookResponse};
use crate::components::dropdown::provider::DropdownProvider;

#[derive(Properties, Clone, PartialEq)]
struct DropdownWrapperProps {
  children: Children,
  #[prop_or_default]
  class: Classes,
}
#[function_component(DropdownWrapper)]
fn dropdown_wrapper(props: &DropdownWrapperProps) -> Html {
  let HookResponse {  dropdown_ref } = use_dropdown();
  html! {
    <div class={classes!("Dropdown", &props.class)} ref={dropdown_ref}>
      {props.children.clone()}
    </div>
  }
}
#[derive(PartialEq, Properties, Clone)]
pub(crate) struct DropDownProps {
  pub children: Children,
  #[prop_or_default]
  pub class: Classes,
}

#[function_component(Dropdown)]
pub(crate) fn dropdown(props: &DropDownProps) -> Html {
  html! {
    <DropdownProvider>
      <DropdownWrapper class={props.class.clone()}>
        { props.children.clone() }
      </DropdownWrapper>
    </DropdownProvider>
  }
}

#[derive(PartialEq, Properties, Clone)]
pub(crate) struct DropdownTriggerProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes
}

#[function_component(DropdownTrigger)]
pub(crate) fn dropdown_trigger(props: &DropdownTriggerProps) -> Html {
  let ctx = use_dropdown_store();

  html! {
    <div
      class={classes!("Dropdown__trigger", &props.class)}
      onclick={{
        let toggle = ctx.toggle.clone();
        Callback::from(move |_| {
          toggle.emit(())
        })
      }}
    >
      { props.children.clone() }
    </div>
  }
}

#[derive(PartialEq, Properties, Clone)]
pub(crate) struct DropdownMenuProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(DropdownPosition::Bottom)]
  pub position: DropdownPosition
}

#[function_component(DropdownMenu)]
pub(crate) fn dropdown_menu(props: &DropdownMenuProps) -> Html {
  let position_class = format!("dropdown-menu-{:?}", props.position).to_lowercase();
  let ctx = use_dropdown_store();

  html! {
    <div class={classes!(
      "Dropdown__menu",
      ctx.is_open.then_some("dropdown-menu-open"),
      position_class,
      &props.class
    )}>
      { props.children.clone() }
    </div>
  }
}

#[derive(PartialEq, Properties, Clone)]
pub(crate) struct DropdownItemProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub on_click: Option<Callback<()>>,
}

#[function_component(DropdownItem)]
pub(crate) fn dropdown_item(props: &DropdownItemProps) -> Html {
  let ctx = use_dropdown_store();
  let on_click = {
    let close = ctx.close.clone();
    let user_on_click = props.on_click.clone();
    Callback::from(move |e: MouseEvent| {
      e.stop_propagation();
      if let Some(callback) = &user_on_click {
        callback.emit(());
      }
      close.emit(());
    })
  };

  html! {
    <div
      class={classes!("Dropdown__item", &props.class)}
      onclick={on_click}
    >
      { props.children.clone() }
    </div>
  }
}