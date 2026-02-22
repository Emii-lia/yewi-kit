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
      default: None
    },
    PropRow {
      name: "color".to_string(),
      r#type: "Color".to_string(),
      description: "Checkbox colour".to_string(),
      default: Some("Color::Primary".to_string())
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
      description: "Input id".to_string(),
      default: None
    },
    PropRow {
      name: "label".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Input label".to_string(),
      default: None
    },
    PropRow {
      name: "name".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Input name".to_string(),
      default: None
    },
    PropRow {
      name: "on_change".to_string(),
      r#type: "Callback<bool>".to_string(),
      description: "Checkbox change action".to_string(),
      default: None
    },
    PropRow {
      name: "size".to_string(),
      r#type: "Size".to_string(),
      description: "Component size".to_string(),
      default: Some("Size::Medium".to_string())
    },
    PropRow {
      name: "value".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Input value".to_string(),
      default: None
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "CheckboxVariant".to_string(),
      description: "Default, Button, Toggle".to_string(),
      default: Some("CheckboxVariant::Default".to_string())
    }
  ]
}