use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "checked".to_string(),
      r#type: "bool".to_string(),
      description: "Checked state".to_string(),
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
      description: "Radio color".to_string(),
      default: Some("Color::Primary".to_string()),
    },
    PropRow {
      name: "disabled".to_string(),
      r#type: "bool".to_string(),
      description: "Disabled state".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "id".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Element ID".to_string(),
      default: None,
    },
    PropRow {
      name: "label".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Radio label text".to_string(),
      default: None,
    },
    PropRow {
      name: "name".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Radio name attribute".to_string(),
      default: None,
    },
    PropRow {
      name: "on_change".to_string(),
      r#type: "Callback<bool>".to_string(),
      description: "Change event handler".to_string(),
      default: None,
    },
    PropRow {
      name: "size".to_string(),
      r#type: "Size".to_string(),
      description: "Radio size".to_string(),
      default: Some("Size::Medium".to_string()),
    },
    PropRow {
      name: "value".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Radio value".to_string(),
      default: None,
    },
  ]
}