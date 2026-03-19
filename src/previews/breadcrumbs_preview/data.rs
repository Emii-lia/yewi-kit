use crate::features::component_table::types::ComponentRow;
use crate::features::prop_table::types::PropRow;

pub(crate) fn get_components() -> Vec<ComponentRow> {
  let breadcrumbs_props: Vec<String> = vec!["children".to_string(), "class".to_string()];
  let breadcrumb_props: Vec<String> = vec!["href".to_string(), "children".to_string(), "class".to_string()];

  vec![
    ComponentRow {
      name: "Breadcrumbs".to_string(),
      description: "Main breadcrumbs container".to_string(),
      props: breadcrumbs_props,
    },
    ComponentRow {
      name: "BreadCrumb<Routable>".to_string(),
      description: "Individual breadcrumb item".to_string(),
      props: breadcrumb_props,
    }
  ]
}

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "children".to_string(),
      r#type: "Html".to_string(),
      description: "Content of the breadcrumb".to_string(),
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
      description: "Route for the breadcrumb link".to_string(),
      default: None
    },
    PropRow {
      name: "separator".to_string(),
      r#type: "String".to_string(),
      description: "Separator between breadcrumbs".to_string(),
      default: Some("String::from('/')".to_string())
    }
  ]
}