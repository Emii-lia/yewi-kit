use crate::features::component_table::types::ComponentRow;
use crate::features::prop_table::types::PropRow;

pub(crate) fn get_components() -> Vec<ComponentRow> {
  vec![
    ComponentRow {
      name: "Carousel".to_string(),
      description: "Main container".to_string(),
      props: vec![
        "children".to_string(),
        "auto_play".to_string(),
        "auto_play_interval".to_string()
      ]
    },
    ComponentRow {
      name: "CarouselContent".to_string(),
      description: "Carousel item container".to_string(),
      props: vec!["children".to_string()]
    },
    ComponentRow {
      name: "CarouselControls".to_string(),
      description: "Control buttons".to_string(),
      props: vec![
        "show_arrows".to_string(),
        "show_dots".to_string(),
        "show_auto_play_toggle".to_string()
      ]
    },
    ComponentRow {
      name: "CarouselItem".to_string(),
      description: "Carousel item container".to_string(),
      props: vec!["id".to_string(), "children".to_string()]
    },
    ComponentRow {
      name: "CarouselWrapper".to_string(),
      description: "Controls and content container".to_string(),
      props: vec!["children".to_string()]
    }
  ]
}

pub(crate) fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "auto_play".to_string(),
      r#type: "bool".to_string(),
      description: "Autoplay carousel slide".to_string(),
      default: Some("false".to_string())
    },
    PropRow {
      name: "auto_play_interval".to_string(),
      r#type: "f64".to_string(),
      description: "Autoplay time interval".to_string(),
      default: Some("3000.0".to_string())
    },
    PropRow {
      name: "children".to_string(),
      r#type: "Children".to_string(),
      description: "Container children".to_string(),
      default: None
    },
    PropRow {
      name: "id".to_string(),
      r#type: "String".to_string(),
      description: "Carousel item id".to_string(),
      default: None
    },
    PropRow {
      name: "show_arrows".to_string(),
      r#type: "bool".to_string(),
      description: "Arrows control visibility".to_string(),
      default: Some("true".to_string())
    },
    PropRow {
      name: "show_auto_play_toggle".to_string(),
      r#type: "bool".to_string(),
      description: "Autoplay toggle visibility".to_string(),
      default: Some("false".to_string())
    },
    PropRow {
      name: "show_dots".to_string(),
      r#type: "bool".to_string(),
      description: "Navigation dots visibility".to_string(),
      default: Some("true".to_string())
    }
  ]
}