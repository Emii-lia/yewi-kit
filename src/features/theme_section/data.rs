#[derive(Clone)]
pub enum ThemeColor {
  Slate,
  Gray,
  Zinc,
  Neutral,
  Stone,
  Emerald,
  Blue,
  Sky,
  Custom(String)
}

impl ThemeColor {

  pub fn to_string(&self) -> String {
    match self {
      ThemeColor::Slate => "Slate".to_string(),
      ThemeColor::Gray => "Gray".to_string(),
      ThemeColor::Zinc => "Zinc".to_string(),
      ThemeColor::Neutral => "Neutral".to_string(),
      ThemeColor::Stone => "Stone".to_string(),
      ThemeColor::Emerald => "Emerald".to_string(),
      ThemeColor::Blue => "Blue".to_string(),
      ThemeColor::Sky => "Sky".to_string(),
      ThemeColor::Custom(hex) => hex.clone(),
    }
  }
  pub fn iter() -> impl Iterator<Item=String> {
    vec![
      ThemeColor::Slate.to_string(),
      ThemeColor::Gray.to_string(),
      ThemeColor::Zinc.to_string(),
      ThemeColor::Neutral.to_string(),
      ThemeColor::Stone.to_string(),
      ThemeColor::Emerald.to_string(),
      ThemeColor::Blue.to_string(),
      ThemeColor::Sky.to_string(),
      ThemeColor::Custom("Custom".to_string()).to_string(),
    ].into_iter()
  }

  pub fn from_str(color: &str) -> Self {
    match color.to_lowercase().as_str() {
      "slate" => ThemeColor::Slate,
      "gray" => ThemeColor::Gray,
      "zinc" => ThemeColor::Zinc,
      "neutral" => ThemeColor::Neutral,
      "stone" => ThemeColor::Stone,
      "emerald" => ThemeColor::Emerald,
      "blue" => ThemeColor::Blue,
      "sky" => ThemeColor::Sky,
      _ => ThemeColor::Custom(color.to_string()),
    }
  }
}
