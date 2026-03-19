use std::any::Any;
use crate::features::prop_table::types::PropRow;

pub fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "anchor_ref".to_string(),
      r#type: "NodeRef".to_string(),
      description: "Component Ref".to_string(),
      default: None
    },
    PropRow {
      name: "children".to_string(),
      r#type: "Html".to_string(),
      description: "Link Content".to_string(),
      default: None
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "href".to_string(),
      r#type: "Routable".to_string(),
      description: "Link redirection".to_string(),
      default: None
    },
    PropRow {
      name: "icon".to_string(),
      r#type: "Option<IconData>".to_string(),
      description: "Link icon".to_string(),
      default: None
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "LinkVariant".to_string(),
      description: "Default, Button".to_string(),
      default: Some("LinkVariant::Default".to_string())
    }
  ]
}