use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "children".to_string(),
      r#type: "Html".to_string(),
      description: "Component children".to_string(),
      default: None,
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None,
    },
    PropRow {
      name: "disabled".to_string(),
      r#type: "bool".to_string(),
      description: "Disbaled state".to_string(),
      default: Some("false".to_string())
    },
    PropRow {
      name: "href".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Button redirection link".to_string(),
      default: None,
    },
    PropRow {
      name: "icon".to_string(),
      r#type: "Option<IconData>".to_string(),
      description: "Button icon".to_string(),
      default: None,
    },
    PropRow {
      name: "is_loading".to_string(),
      r#type: "bool".to_string(),
      description: "Loading state".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "onclick".to_string(),
      r#type: "Callback<MouseEvent>".to_string(),
      description: "Click event".to_string(),
      default: None
    },
    PropRow {
      name: "size".to_string(),
      r#type: "Size".to_string(),
      description: "Button size".to_string(),
      default: Some("Size::Medium".to_string())
    },
    PropRow {
      name: "title".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Title tooltip".to_string(),
      default: None,
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "ButtonVariant".to_string(),
      description: "Primary, Secondary, Tertiary, Success, Danger".to_string(),
      default: Some("ButtonVariant::Primary".to_string()),
    }
  ]
}