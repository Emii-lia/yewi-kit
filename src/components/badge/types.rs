#[derive(PartialEq, Clone, Debug)]
pub(crate) enum BadgeColor {
  None,
  Purple,
  Blue,
  Green,
  Red,
  Yellow,
  Zinc,
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

#[derive(PartialEq, Clone, Debug)]
pub(crate) enum BadgeVariant {
  Default,
  Plain,
  Filled
}