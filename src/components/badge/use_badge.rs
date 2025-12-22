use yew::{hook, Properties};
use crate::components::{BadgeColor, BadgeVariant};

#[derive(PartialEq, Properties, Clone)]
pub struct HookParams {
  pub color: BadgeColor,
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
  
  let variant = format!("{:?}", variant.clone()).to_lowercase();
  
  let color_class = format!("{:?}", color.clone()).to_lowercase();
  
  HookResponse {
    variant,
    color: color_class,
  }
}