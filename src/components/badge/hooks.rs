use yew::{hook, Properties};
use crate::components::{BadgeVariant};
use crate::types::Color;

#[derive(PartialEq, Properties, Clone)]
pub struct HookParams {
  pub color: Color,
  pub variant: BadgeVariant,
}

#[derive(Clone, PartialEq)]
pub struct HookResponse {
  pub color: String,
  pub variant: String,
}

#[hook]
pub(crate) fn use_badge(params: HookParams) -> HookResponse {
  let HookParams { color, variant } = params;
  
  let variant = format!("badge-{:?}", variant).to_lowercase();
  
  let color_class = format!("badge-{:?}", color).to_lowercase();
  
  HookResponse {
    variant,
    color: color_class,
  }
}