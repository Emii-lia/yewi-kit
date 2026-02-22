use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "children".to_string(),
      r#type: "Children".to_string(),
      description: "Divider content".to_string(),
      default: None
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "vertical".to_string(),
      r#type: "bool".to_string(),
      description: "Vertical display".to_string(),
      default: Some("false".to_string())
    }
  ]
}