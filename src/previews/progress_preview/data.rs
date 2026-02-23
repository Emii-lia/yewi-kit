use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "as_fraction".to_string(),
      r#type: "bool".to_string(),
      description: "Progress value as fraction".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None,
    },
    PropRow {
      name: "color".to_string(),
      r#type: "Color".to_string(),
      description: "Progress bar color".to_string(),
      default: Some("Color::Primary".to_string()),
    },
    PropRow {
      name: "height".to_string(),
      r#type: "u32".to_string(),
      description: "Progress bar height".to_string(),
      default: Some("10".to_string()),
    },
    PropRow {
      name: "max".to_string(),
      r#type: "u32".to_string(),
      description: "Maximum value".to_string(),
      default: Some("100".to_string()),
    },
    PropRow {
      name: "radial_size".to_string(),
      r#type: "Size".to_string(),
      description: "Size of radial progress".to_string(),
      default: Some("Size::Medium".to_string()),
    },
    PropRow {
      name: "show_percentage".to_string(),
      r#type: "bool".to_string(),
      description: "Show value".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "value".to_string(),
      r#type: "u32".to_string(),
      description: "Current progress value".to_string(),
      default: Some("0".to_string()),
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "ProgressVariant".to_string(),
      description: "Linear, Radial".to_string(),
      default: Some("ProgressVariant::Linear".to_string()),
    },
  ]
}