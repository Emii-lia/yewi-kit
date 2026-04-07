use crate::features::prop_table::types::PropRow;

pub fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "file".to_string(),
      r#type: "Option<FileValue>".to_string(),
      description: "File<File>, Url<String, Option<String>, Option<String>>".to_string(),
      default: None
    },
    PropRow {
      name: "onclick".to_string(),
      r#type: "Callback<()>".to_string(),
      description: "Preview click callback".to_string(),
      default: None
    },
    PropRow {
      name: "onremove".to_string(),
      r#type: "Callback<()>".to_string(),
      description: "Remove file callback".to_string(),
      default: None
    }
  ]
}