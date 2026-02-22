use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec! [
    PropRow {
      name: "children".to_string(),
      r#type: "Html".to_string(),
      description: "Collapse content".to_string(),
      default: None,
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None,
    },
    PropRow {
      name: "summary".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Trigger content".to_string(),
      default: None,
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "CollapseVariant".to_string(),
      description: "Focus, Toggle".to_string(),
      default: Some("CollapseVariant::Focus".to_string())
    }
  ]
}