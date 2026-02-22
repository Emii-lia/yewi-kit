use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "children".to_string(),
      r#type: "ChildrenWithProps<Avatar>".to_string(),
      description: "Avatar components within AvatarGroup".to_string(),
      default: None
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "color".to_string(),
      r#type: "Color".to_string(),
      description: "Avatar children color".to_string(),
      default: Some("Color::Primary".to_string()),
    },
    PropRow {
      name: "max".to_string(),
      r#type: "usize".to_string(),
      description: "Maximum children to show".to_string(),
      default: Some("4".to_string()),
    },
    PropRow {
      name: "rounded".to_string(),
      r#type: "bool".to_string(),
      description: "Avtar children radius".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "size".to_string(),
      r#type: "Size".to_string(),
      description: "Avatar children size".to_string(),
      default: Some("Size::Medium".to_string()),
    },
    PropRow {
      name: "variant".to_string(),
      r#type: "AvatarGroupVariant".to_string(),
      description: "Linear, Stacked".to_string(),
      default: Some("AvatarGroupVariant::Linear".to_string()),
    },
    PropRow {
      name: "with_border".to_string(),
      r#type: "bool".to_string(),
      description: "Avatar children border".to_string(),
      default: Some("false".to_string()),
    },

  ]
}