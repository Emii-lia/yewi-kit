use crate::features::component_table::types::ComponentRow;
use crate::features::prop_table::types::PropRow;

pub(crate) fn get_components() -> Vec<ComponentRow> {
  let t_props = vec!["children".to_string(), "class".to_string()];
  vec![
    ComponentRow {
      name: "Table".to_string(),
      description: "Main container for table structure".to_string(),
      props: vec!["children".to_string(), "class".to_string(), "variant".to_string()],
    },
    ComponentRow {
      name: "TableBody".to_string(),
      description: "Table body".to_string(),
      props: t_props.clone(),
    },
    ComponentRow {
      name: "TableDataCell".to_string(),
      description: "Table body cell".to_string(),
      props: t_props.clone(),
    },
    ComponentRow {
      name: "TableHead".to_string(),
      description: "Table header".to_string(),
      props: t_props.clone(),
    },
    ComponentRow {
      name: "TableHeaderCell".to_string(),
      description: "Table header cell".to_string(),
      props: t_props.clone(),
    },
    ComponentRow {
      name: "TableRow".to_string(),
      description: "Table row".to_string(),
      props: t_props.clone(),
    },
  ]
}

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "children".to_string(),
      r#type: "Children".to_string(),
      description: "Content".to_string(),
      default: None,
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom classes".to_string(),
      default: None,
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "TableVariant".to_string(),
      description: "Default, Stripped, Bordered".to_string(),
      default: Some("TableVariant::Default".to_string()),
    }
  ]
}