use crate::features::component_table::types::ComponentRow;
use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec! [
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
      name: "icon".to_string(),
      r#type: "Option<IconData>".to_string(),
      description: "Trigger icon".to_string(),
      default: None,
    },
    PropRow {
      name: "indicator".to_string(),
      r#type: "CollapseIndicator".to_string(),
      description: "Plus, Chevron".to_string(),
      default: Some("CollapseIndicator::Plus".to_string())
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "CollapseVariant".to_string(),
      description: "Focus, Toggle(bool)".to_string(),
      default: Some("CollapseVariant::Focus".to_string())
    }
  ]
}

pub(crate) fn get_components() -> Vec<ComponentRow> {
  vec![
    ComponentRow {
      name: "Collapse".to_string(),
      description: "Collapse main container".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
        "variant".to_string(),
      ]
    },
    ComponentRow {
      name: "CollapseTrigger".to_string(),
      description: "Collapse trigger".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
        "icon".to_string(),
        "indicator".to_string(),
      ]
    },
    ComponentRow {
      name: "CollapseContent".to_string(),
      description: "Collapse content".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
      ]
    }
  ]
}