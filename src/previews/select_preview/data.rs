use crate::components::SelectOption;
use crate::features::prop_table::types::PropRow;

pub fn options() -> Vec<SelectOption> {
  let mut options = Vec::new();
  
options.push(SelectOption::Simple("Option 1".into()));
options.push(SelectOption::Simple("Option 2".into()));
options.push(SelectOption::Pair {
  label: "Option 3 Label".into(),
  value: "option_3_value".into(),
});
  options.push(SelectOption::Pair {
    label: "Option 4 Label".into(),
    value: "option_4_value".into(),
  });
  options
}

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None,
    },
    PropRow {
      name: "disabled".to_string(),
      r#type: "bool".to_string(),
      description: "Disabled state".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "errors".to_string(),
      r#type: "Vec<String>".to_string(),
      description: "Error messages".to_string(),
      default: None,
    },
    PropRow {
      name: "label".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Select label".to_string(),
      default: None,
    },
    PropRow {
      name: "main_class".to_string(),
      r#type: "Classes".to_string(),
      description: "Main container custom class".to_string(),
      default: None,
    },
    PropRow {
      name: "name".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Select name attribute".to_string(),
      default: None,
    },
    PropRow {
      name: "on_change".to_string(),
      r#type: "Option<Callback<Event>>".to_string(),
      description: "Change event handler".to_string(),
      default: None,
    },
    PropRow {
      name: "options".to_string(),
      r#type: "Vec<SelectOption>".to_string(),
      description: "Select options".to_string(),
      default: None,
    },
    PropRow {
      name: "size".to_string(),
      r#type: "Size".to_string(),
      description: "Select size".to_string(),
      default: Some("Size::Medium".to_string()),
    },
    PropRow {
      name: "value".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Select value".to_string(),
      default: None,
    },
  ]
}