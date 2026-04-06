use yew_icons::IconData;
use crate::components::button::ButtonVariant;
use crate::types::Size;

#[derive(Clone, PartialEq)]
pub enum FileInputType {
  Button(ButtonVariant, Size, Option<IconData>),
  Input,
  DnD
}

impl FileInputType {
  pub fn button() -> Self {
    Self::Button(ButtonVariant::Primary, Size::Medium, None)
  }
  pub fn with_variant(self, variant: ButtonVariant) -> Self {
    match self {
      Self::Input => Self::Input,
      Self::DnD => Self::DnD,
      Self::Button(_, _, icon) => Self::Button(variant, Size::Medium, icon),
    }
  }
  pub fn with_size(self, size: Size) -> Self {
    match self {
      Self::Input => Self::Input,
      Self::DnD => Self::DnD,
      Self::Button(variant, _, icon) => Self::Button(variant, size, icon),
    }
  }
  
  pub fn with_icon(self, icon: Option<IconData>) -> Self {
    match self {
      Self::Input => Self::Input,
      Self::DnD => Self::DnD,
      Self::Button(variant, size, _) => Self::Button(variant, size, icon),
    } 
  }
}