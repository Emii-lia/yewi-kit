#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CollapseVariant {
  Toggle,
  Focus
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CollapseIndicator {
  Plus,
  Chevron
}

impl CollapseIndicator {
  pub fn as_str(&self) -> &'static str {
    match self {
      CollapseIndicator::Plus => "plus",
      CollapseIndicator::Chevron => "chevron",
    }
  }
}