use crate::features::component_table::types::ComponentRow;
use crate::features::prop_table::types::PropRow;

pub(crate) fn get_components() -> Vec<ComponentRow> {
  vec![
    ComponentRow {
      name: "Dropdown".to_string(),
      description: "Main container".to_string(),
      props: vec!["children".to_string(), "class".to_string()],
    },
    ComponentRow {
      name: "DropdownItem".to_string(),
      description: "Dropdown menu item".to_string(),
      props: vec!["children".to_string(), "class".to_string(), "on_click".to_string()],
    },
    ComponentRow {
      name: "DropdownMenu".to_string(),
      description: "Menu container".to_string(),
      props: vec!["children".to_string(), "class".to_string(), "position".to_string()],
    },
    ComponentRow {
      name: "DropdownTrigger".to_string(),
      description: "Trigger container".to_string(),
      props: vec!["children".to_string(), "class".to_string()],
    }
  ]
}

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "children".to_string(),
      r#type: "Html".to_string(),
      description: "Container children".to_string(),
      default: None
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "position".to_string(),
      r#type: "DropdownPosition".to_string(),
      description: "Top, Bottom".to_string(),
      default: Some("DropdownPosition::Bottom".to_string())
    },
    PropRow {
      name: "on_click".to_string(),
      r#type: "Option<Callback<()>>".to_string(),
      description: "Item click action".to_string(),
      default: None
    }
  ]
}