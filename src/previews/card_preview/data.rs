use crate::features::component_table::types::ComponentRow;
use crate::features::prop_table::types::PropRow;

pub(crate) fn get_components() -> Vec<ComponentRow> {
  let c_props: Vec<String> = vec!["children".to_string(), "class".to_string()];

  vec![
    ComponentRow {
      name: "Card".to_string(),
      description: "Main container".to_string(),
      props: c_props.clone(),
    },
    ComponentRow {
      name: "CardHeader".to_string(),
      description: "Header container".to_string(),
      props: c_props.clone(),
    },
    ComponentRow {
      name: "CardAction".to_string(),
      description: "Header action container".to_string(),
      props: c_props.clone(),
    },
    ComponentRow {
      name: "CardTitle".to_string(),
      description: "Container title".to_string(),
      props: c_props.clone(),
    },
    ComponentRow {
      name: "CardDescription".to_string(),
      description: "Container description".to_string(),
      props: c_props.clone(),
    },
    ComponentRow {
      name: "CardContent".to_string(),
      description: "Body container".to_string(),
      props: c_props.clone(),
    },
    ComponentRow {
      name: "CardFooter".to_string(),
      description: "Footer container".to_string(),
      props: c_props.clone(),
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
    }
  ]
}