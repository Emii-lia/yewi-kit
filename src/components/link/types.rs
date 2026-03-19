#[derive(PartialEq, Clone)]
pub enum LinkVariant {
  Default,
  Button
}

impl LinkVariant {
  pub fn as_str(&self) -> &'static str {
    match self {
      LinkVariant::Default => "default",
      LinkVariant::Button => "button"
    }
  }
}