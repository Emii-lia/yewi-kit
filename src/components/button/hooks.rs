use yew::{Callback, MouseEvent, Properties, prelude::*};
use crate::components::ButtonVariant;
use crate::types::{Size};

#[derive(Properties, PartialEq)]
pub struct HookParams {
  pub variant: ButtonVariant,
  pub size: Size,
  pub onclick: Callback<MouseEvent>
}

#[derive(Clone, PartialEq)]
pub struct HookResponse {
  pub variant: String,
  pub size: String,
  pub on_click: Callback<MouseEvent>
}

#[hook]
pub(crate) fn use_button(params: HookParams) -> HookResponse {
  let HookParams { variant, size, onclick } = params;
  let onclick = onclick;
  let on_click = Callback::from(move |e: MouseEvent| {
    onclick.emit(e);
  });

  let variant = match variant {
    ButtonVariant::Primary => "btn-primary",
    ButtonVariant::Secondary => "btn-secondary",
    ButtonVariant::Tertiary => "btn-tertiary",
    ButtonVariant::Danger => "btn-danger",
    ButtonVariant::Success => "btn-success",
  };

  let size = match size {
    Size::Small => "btn-sm",
    Size::Medium => "btn-md",
    Size::Large => "btn-lg",
  };

  HookResponse {
    variant: variant.to_string(),
    size: size.to_string(),
    on_click
  }
}