use crate::features::prop_table::types::PropRow;

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "autocomplete".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Autocomplete attribute".to_string(),
      default: None,
    },
    PropRow {
      name: "autofocus".to_string(),
      r#type: "bool".to_string(),
      description: "Autofocus state".to_string(),
      default: Some("false".to_string()),
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
      name: "id".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Element ID".to_string(),
      default: None,
    },
    PropRow {
      name: "input_size".to_string(),
      r#type: "Size".to_string(),
      description: "Input size".to_string(),
      default: Some("Size::Medium".to_string()),
    },
    PropRow {
      name: "max".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Maximum value".to_string(),
      default: None,
    },
    PropRow {
      name: "maxlength".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Maximum length".to_string(),
      default: None,
    },
    PropRow {
      name: "min".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Minimum value".to_string(),
      default: None,
    },
    PropRow {
      name: "minlength".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Minimum length".to_string(),
      default: None,
    },
    PropRow {
      name: "name".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Input name".to_string(),
      default: None,
    },
    PropRow {
      name: "node_ref".to_string(),
      r#type: "NodeRef".to_string(),
      description: "Reference to DOM node".to_string(),
      default: None,
    },
    PropRow {
      name: "onblur".to_string(),
      r#type: "Callback<FocusEvent>".to_string(),
      description: "Blur event handler".to_string(),
      default: None,
    },
    PropRow {
      name: "onchange".to_string(),
      r#type: "Callback<Event>".to_string(),
      description: "Change event handler".to_string(),
      default: None,
    },
    PropRow {
      name: "onfocus".to_string(),
      r#type: "Callback<FocusEvent>".to_string(),
      description: "Focus event handler".to_string(),
      default: None,
    },
    PropRow {
      name: "oninput".to_string(),
      r#type: "Callback<InputEvent>".to_string(),
      description: "Input event handler".to_string(),
      default: None,
    },
    PropRow {
      name: "onkeydown".to_string(),
      r#type: "Callback<KeyboardEvent>".to_string(),
      description: "Keydown event handler".to_string(),
      default: None,
    },
    PropRow {
      name: "onkeypress".to_string(),
      r#type: "Callback<KeyboardEvent>".to_string(),
      description: "Keypress event handler".to_string(),
      default: None,
    },
    PropRow {
      name: "onkeyup".to_string(),
      r#type: "Callback<KeyboardEvent>".to_string(),
      description: "Keyup event handler".to_string(),
      default: None,
    },
    PropRow {
      name: "pattern".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Validation pattern".to_string(),
      default: None,
    },
    PropRow {
      name: "placeholder".to_string(),
      r#type: "AttrValue".to_string(),
      description: "Placeholder text".to_string(),
      default: None,
    },
    PropRow {
      name: "readonly".to_string(),
      r#type: "bool".to_string(),
      description: "Read-only state".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "required".to_string(),
      r#type: "bool".to_string(),
      description: "Required state".to_string(),
      default: Some("false".to_string()),
    },
    PropRow {
      name: "step".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Step value".to_string(),
      default: None,
    },
    PropRow {
      name: "tabindex".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Tab index".to_string(),
      default: None,
    },
    PropRow {
      name: "value".to_string(),
      r#type: "Option<AttrValue>".to_string(),
      description: "Input value".to_string(),
      default: None,
    },
  ]
}