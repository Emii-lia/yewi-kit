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
      name: "count".to_string(),
      r#type: "usize".to_string(),
      description: "Total number of pages".to_string(),
      default: None
    },
    PropRow {
      name: "current".to_string(),
      r#type: "usize".to_string(),
      description: "Current active page number".to_string(),
      default: Some("1".to_string()),
    },
    PropRow {
      name: "on_change".to_string(),
      r#type: "Callback<usize>".to_string(),
      description: "Callback fired when page changes, receives the new page number".to_string(),
      default: None
    },
    PropRow {
      name: "rounded".to_string(),
      r#type: "bool".to_string(),
      description: "Rounded pagination button".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "size".to_string(),
      r#type: "Size".to_string(),
      description: "Size of the pagination buttons".to_string(),
      default: Some("Size::Medium".to_string()),
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "PaginationVariant".to_string(),
      description: "Default, Outlined".to_string(),
      default: Some("PaginationVariant::Default".to_string()),
    },
  ]
}