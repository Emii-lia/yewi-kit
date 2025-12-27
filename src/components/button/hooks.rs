use yew::{Callback, MouseEvent, Properties, prelude::*};
use crate::types::{Size, ButtonVariant};

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
    ButtonVariant::Primary => "primary",
    ButtonVariant::Secondary => "secondary",
    ButtonVariant::Tertiary => "tertiary",
    ButtonVariant::Danger => "danger",
    ButtonVariant::Success => "success",
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