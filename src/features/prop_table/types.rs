#[derive(Clone, PartialEq)]
pub struct PropRow  {
  pub name: String,
  pub r#type: String,
  pub description: String,
  pub default: Option<String>
}