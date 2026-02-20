use crate::features::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "alt".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Fallback value of the avatar if the image failed to load".to_string(),
      default: None
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "color".to_string(),
      r#type: "Color".to_string(),
      description: "Background or border color if on fallback or with border".to_string(),
      default: Some("Color::Primary".to_string()),
    },
    PropRow {
      name: "no_split".to_string(),
      r#type: "bool".to_string(),
      description: "Use fallback whole value or just the initials".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "rounded".to_string(),
      r#type: "bool".to_string(),
      description: "Rounded avatar".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "size".to_string(),
      r#type: "Size".to_string(),
      description: "Avatar Size".to_string(),
      default: Some("Size::Medium".to_string()),
    },
    PropRow {
      name: "style".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Component custom style".to_string(),
      default: None,
    },
    PropRow {
      name: "src".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Image source".to_string(),
      default: None
    },
    PropRow {
      name: "take".to_string(),
      r#type: "usize".to_string(),
      description: "The number of initials to take from the fallback value (doesn't apply if no_split is set to true)".to_string(),
      default: Some("2".to_string())
    },
    PropRow {
      name: "title".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Component title tooltip".to_string(),
      default: None
    },
    PropRow {
      name: "with_border".to_string(),
      r#type: "bool".to_string(),
      description: "Avatar border".to_string(),
      default: Some("false".to_string()),
    },
  ]
}