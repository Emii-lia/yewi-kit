use crate::features::component_table::types::ComponentRow;
use crate::features::prop_table::types::PropRow;

pub(crate) fn get_components() -> Vec<ComponentRow> {
  vec![
    ComponentRow {
      name: "Tab".to_string(),
      description: "Tab item".to_string(),
      props: vec![
        "children".to_string(),
        "classes".to_string(),
        "label".to_string(),
        "value".to_string(),
      ]
    },
    ComponentRow {
      name: "Tabs".to_string(),
      description: "Tabs container".to_string(),
      props: vec![
        "children".to_string(),
        "active_tab".to_string(),
        "classes".to_string(),
        "on_tab_change".to_string(),
        "color".to_string(),
      ]
    }
  ]
}

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "active_tab".to_string(),
      r#type: "Option<String>".to_string(),
      description: "Active tab value".to_string(),
      default: None
    },
    PropRow {
      name: "children (Tab)".to_string(),
      r#type: "Children".to_string(),
      description: "Tab content".to_string(),
      default: None
    },
    PropRow {
      name: "children (Tabs)".to_string(),
      r#type: "ChildrenWithProps<Tab>".to_string(),
      description: "Tabs content".to_string(),
      default: None
    },
    PropRow {
      name: "classes".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "color".to_string(),
      r#type: "Color".to_string(),
      description: "Tabs colour".to_string(),
      default: Some("Color::Primary".to_string())
    },
    PropRow {
      name: "label".to_string(),
      r#type: "Html".to_string(),
      description: "Tab label".to_string(),
      default: None
    },
    PropRow {
      name: "on_tab_change".to_string(),
      r#type: "Callback<String>".to_string(),
      description: "Tab change action".to_string(),
      default: None
    },
    PropRow {
      name: "value".to_string(),
      r#type: "String".to_string(),
      description: "Tab value".to_string(),
      default: None
    }
  ]
}