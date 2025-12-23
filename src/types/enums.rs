#[derive(PartialEq, Clone)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Tertiary,
    Danger,
    Success,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Size {
    Small,
    Medium,
    Large,
}

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Color {
  None,
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