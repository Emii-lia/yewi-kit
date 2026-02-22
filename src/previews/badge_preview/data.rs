use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "color".to_string(),
      r#type: "Color".to_string(),
      description: "Badge colour".to_string(),
      default: Some("Color::Primary".to_string()),
    },
    PropRow {
      name: "icon".to_string(),
      r#type: "Option<IconData>".to_string(),
      description: "Badge icon".to_string(),
      default: None
    },
    PropRow {
      name: "label".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Badge content".to_string(),
      default: None
    },
    PropRow {
      name: "rounded".to_string(),
      r#type: "bool".to_string(),
      description: "Rounded badge".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "title".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Title tooltip".to_string(),
      default: None
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "BadgeVariant".to_string(),
      description: "Default, Plain, Filled".to_string(),
      default: Some("BadgeVariant::Default".to_string()),
    },
    PropRow {
      name: "with_border".to_string(),
      r#type: "bool".to_string(),
      description: "Badge border".to_string(),
      default: Some("false".to_string()),
    }
  ]
}