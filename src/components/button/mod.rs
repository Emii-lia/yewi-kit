mod hooks;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, MouseEvent, Properties};
use crate::components::button::hooks::{use_button, HookParams, HookResponse};
use crate::types::{Size, ButtonVariant};

#[derive(Clone, PartialEq)]
pub enum ButtonClick {
  Mouse(Callback<MouseEvent>),
  Simple(Callback<()>)
}
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
pub(crate) fn button(Props {
  href,
  children,
  is_loading,
  class,
  disabled,
  onclick,
  variant,
  size
}: &Props) -> Html {
  
  let HookResponse {
    variant,
    size,
    on_click
  } = use_button(HookParams {
    size: size.clone(),
    variant: variant.clone(),
    onclick: onclick.clone()
  });

  html! {
    if Some(href) != None && href != "" {
      <a
        href={href}
        class={classes!("Button", variant, size, (*is_loading).then_some("loading"), class)}
        disabled={*disabled || *is_loading}
      >
        {children.clone()}
      </a>
    } else {
      <button
        class={classes!("Button", variant, size, (*is_loading).then_some("loading"), class)}
        disabled={*disabled || *is_loading}
        onclick={on_click}
      >
        {
          html! {
            if *is_loading {
              <div class="spinner-container">
                <span class="spinner"></span>
              </div>
            } else {
              {children.clone()}
            }
          }
        }
      </button>
    }
  }
}