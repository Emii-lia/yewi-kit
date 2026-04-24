mod types;
mod hooks;

use yew::{classes, component, html, Callback, Classes, Html, Properties};
use yew_icons::{Icon, IconData};
pub(crate) use types::*;
use crate::components::collapse::hooks::{use_collapse, HookParams, HookResponse};

#[derive(Properties, Clone, PartialEq)]
pub struct CollapseProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or(CollapseVariant::Focus)]
  pub variant: CollapseVariant,
}

#[derive(Properties, Clone, PartialEq)]
pub struct CollapseTriggerProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub icon: Option<IconData>,
  #[prop_or(CollapseIndicator::Plus)]
  pub indicator: CollapseIndicator
}

#[derive(Properties, Clone, PartialEq)]
pub struct CollapseContentProps {
  pub children: Html,
  #[prop_or_default]
  pub class: Classes,
}
#[component(CollapseTrigger)]
pub(crate) fn collapse_trigger(props: &CollapseTriggerProps) -> Html {
  html! {
    <div
      class={classes!("CollapseTrigger", &props.class)}
      data-indicator={props.indicator.as_str()}
    >
      <div class={"collapse-trigger-content"}>
        {html! {
          if let Some(icon) = props.icon {
            <Icon data={icon} class="collapse-trigger-icon"/>
          }
        }}
        {props.children.clone()}
      </div>
    </div>
  }
}

#[component(CollapseContent)]
pub(crate) fn collapse_content(props: &CollapseContentProps) -> Html {
  html! {
    <div class={classes!("CollapseContent", &props.class)}>
      {props.children.clone()}
    </div>
  }
}

#[component(Collapse)]
pub(crate) fn collapse(props: &CollapseProps) -> Html {
  
  let HookResponse { is_open, toggle,checkbox_ref } = use_collapse(HookParams { variant: props.variant.clone() });

  match props.variant {
    CollapseVariant::Toggle(_) => {
      
      html! {
        <div
          class={classes!("Collapse", "toggle", is_open.then_some("open"), &props.class)}
          onclick={{
            let toggle = toggle.clone();
            Callback::from(move |_e| {
              toggle.emit(());
            })
          }}
        >
          <input
            type={"checkbox"}
            class={"collapse-state"}
            onchange={{
              let toggle = toggle.clone();
              Callback::from(move |_| {
                toggle.emit(());
              })
            }}
            checked={is_open}
            ref={checkbox_ref.clone()}
          />
          {props.children.clone()}
        </div>
      }
    },
    CollapseVariant::Focus => {
      html! {
        <div class={classes!("Collapse", &props.class)} tabindex={"1"}>
          {props.children.clone()}
        </div>
      }
    }
  }
}