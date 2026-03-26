use crate::types::Size;

#[derive(PartialEq, Clone)]
pub enum LinkVariant {
  Default,
  Button(LinkButtonVariant, Size)
}

#[derive(PartialEq, Clone)]
pub enum LinkButtonVariant {
  Primary,
  Secondary,
  Tertiary,
  Danger,
  Success
}

impl LinkButtonVariant {
  pub fn as_str(&self) -> &'static str {
    match self {
      LinkButtonVariant::Primary => "primary",
      LinkButtonVariant::Secondary => "secondary",
      LinkButtonVariant::Tertiary => "tertiary",
      LinkButtonVariant::Danger => "danger",
      LinkButtonVariant::Success => "success",
    }
  }
}

impl Default for LinkVariant {
  fn default() -> Self {
    Self::Default
  }
}

impl LinkVariant {
  pub fn as_str(&self) -> &'static str {
    match self {
      LinkVariant::Default => "default",
      LinkVariant::Button(variant, size) => {
        let str = format!("l-btn l-btn-{} l-btn-{}", variant.as_str(), size.as_str());
        Box::leak(str.into_boxed_str())
      }
    }
  }

  pub fn primary() -> Self {
    Self::Button(LinkButtonVariant::Primary, Size::Medium)
  }
  pub fn secondary() -> Self {
    Self::Button(LinkButtonVariant::Secondary, Size::Medium)
  }
  pub fn tertiary() -> Self {
    Self::Button(LinkButtonVariant::Tertiary, Size::Medium)
  }
  pub fn danger() -> Self {
    Self::Button(LinkButtonVariant::Danger, Size::Medium)
  }
  pub fn success() -> Self {
    Self::Button(LinkButtonVariant::Success, Size::Medium)
  }
  pub fn with_size(self, size: Size) -> Self {
    match self {
      LinkVariant::Default => self,
      LinkVariant::Button(variant, _) => LinkVariant::Button(variant, size)
    }
  }
}