#[derive(PartialEq, Clone, Debug)]
pub enum Size {
    Small,
    Medium,
    Large,
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Color {
  Primary,
  Purple,
  Blue,
  Green,
  Red,
  Yellow,
  Gray,
  Orange,
  Pink,
  Fuchsia,
  Cyan,
  Teal,
  Indigo,
  Violet,
  Emerald,
  Lime,
  Amber,
  Rose,
  Sky,
  Light,
  Dark,
  Transparent,
}

impl Size {
  pub fn as_str(&self) -> &'static str {
    match self {
      Size::Small => "small",
      Size::Medium => "medium",
      Size::Large => "large",
    }
  }
}