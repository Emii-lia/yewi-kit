use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None,
    },
    PropRow {
      name: "cols".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Number of columns".to_string(),
      default: None,
    },
    PropRow {
      name: "disabled".to_string(),
      r#type: "bool".to_string(),
      description: "Disabled state".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "errors".to_string(),
      r#type: "Vec<String>".to_string(),
      description: "Error messages".to_string(),
      default: None,
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
      description: "Textarea label".to_string(),
      default: None,
    },
    PropRow {
      name: "max_length".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Maximum length".to_string(),
      default: None,
    },
    PropRow {
      name: "name".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Textarea name attribute".to_string(),
      default: None,
    },
    PropRow {
      name: "on_change".to_string(),
      r#type: "Callback<Event>".to_string(),
      description: "Change event".to_string(),
      default: None,
    },
    PropRow {
      name: "placeholder".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Placeholder text".to_string(),
      default: None,
    },
    PropRow {
      name: "resize".to_string(),
      r#type: "bool".to_string(),
      description: "Allow resizing".to_string(),
      default: Some("true".to_string()),
    },
    PropRow {
      name: "rows".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Number of rows".to_string(),
      default: None,
    },
    PropRow {
      name: "value".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Textarea value".to_string(),
      default: None,
    },
  ]
}