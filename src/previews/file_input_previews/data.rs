use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "accept".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Accepted file extensions".to_string(),
      default: None
    },
    PropRow {
      name: "button_size".to_string(),
      r#type: "Size".to_string(),
      description: "Input button type size".to_string(),
      default: Some("Size::Medium".to_string())
    },
    PropRow {
      name: "button_variant".to_string(),
      r#type: "ButtonVariant".to_string(),
      description: "Input button type variant".to_string(),
      default: Some("ButtonVariant::Primary".to_string())
    },
    PropRow {
      name: "children".to_string(),
      r#type: "Html".to_string(),
      description: "Content".to_string(),
      default: None
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "disabled".to_string(),
      r#type: "bool".to_string(),
      description: "Disable input".to_string(),
      default: Some("false".to_string())
    },
    PropRow {
      name: "multiple".to_string(),
      r#type: "bool".to_string(),
      description: "Accept multiple".to_string(),
      default: Some("false".to_string())
    },
    PropRow {
      name: "onchange".to_string(),
      r#type: "Callback<FileList>".to_string(),
      description: "Callback on file change".to_string(),
      default: None
    },
    PropRow {
      name: "r#type".to_string(),
      r#type: "FileInputType".to_string(),
      description: "Input, Button, DnD".to_string(),
      default: Some("FileInputType::Input".to_string())
    },
    PropRow {
      name: "value".to_string(),
      r#type: "Option<FileList>".to_string(),
      description: "Initial value".to_string(),
      default: None
    }
  ]
}