mod hooks;
mod types;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, MouseEvent, Properties};
use crate::components::button::hooks::{use_button, HookParams, HookResponse};
use crate::types::{Size};
pub use types::*;

#[derive(Properties, PartialEq)]
pub struct Props {
  pub children: Html,
  #[prop_or(ButtonVariant::Primary)]
  pub variant: ButtonVariant,
  #[prop_or_default]
  pub class: Classes,
  #[prop_or_default]
  pub onclick: Callback<MouseEvent>,
  #[prop_or_default]
  pub disabled: bool,
  #[prop_or(Size::Medium)]
  pub size: Size,
  #[prop_or_default]
  pub href: AttrValue,
  #[prop_or_default]
  pub is_loading: bool,
}
#[function_component(Button)]
pub(crate) fn button(props: &Props) -> Html {
  
  let HookResponse {
    variant,
    size,
    on_click
  } = use_button(HookParams {
    size: props.size.clone(),
    variant: props.variant.clone(),
    onclick: props.onclick.clone()
  });

  html! {
    if Some(&props.href) != None && &props.href != "" {
      <a
        href={&props.href}
        class={classes!("Button", variant, size, props.is_loading.then_some("loading"), &props.class)}
        disabled={props.disabled || props.is_loading}
      >
        {props.children.clone()}
      </a>
    } else {
      <button
        class={classes!("Button", variant, size, props.is_loading.then_some("loading"), &props.class)}
        disabled={props.disabled || props.is_loading}
        onclick={on_click}
      >
        {
          html! {
            if props.is_loading {
              <div class="spinner-container">
                <span class="spinner"></span>
              </div>
            } else {
              {props.children.clone()}
            }
          }
        }
      </button>
    }
  }
}