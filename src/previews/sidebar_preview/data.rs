use crate::features::component_table::types::ComponentRow;
use crate::features::prop_table::types::PropRow;

pub fn get_components() -> Vec<ComponentRow> {
  vec![
    ComponentRow {
      name: "SidebarProvider".to_string(),
      description: "Sidebar context provider".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
        "default_open".to_string(),
        "onopenchange".to_string(),
        "open".to_string(),
        "style".to_string(),
      ]
    },
    ComponentRow {
      name: "Sidebar".to_string(),
      description: "Sidebar main container".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
        "position".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarTrigger".to_string(),
      description: "Sidebar trigger button".to_string(),
      props: vec![
        "class".to_string(),
        "icon".to_string(),
        "onclick".to_string(),
        "size".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarHeader".to_string(),
      description: "Sidebar header".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarTitle".to_string(),
      description: "Sidebar header title".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarFooter".to_string(),
      description: "Sidebar footer".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarContent".to_string(),
      description: "Sidebar content body".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarGroup".to_string(),
      description: "Sidebar menu group".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarGroupTitle".to_string(),
      description: "Sidebar group title".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarGroupContent".to_string(),
      description: "Sidebar group content".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarMenu".to_string(),
      description: "Sidebar menu container".to_string(),
      props: vec![
        "children".to_string(),
        "class".to_string(),
      ]
    },
    ComponentRow {
      name: "SidebarMenuItem".to_string(),
      description: "Sidebar menu item".to_string(),
      props: vec![
        // "action".to_string(),
        "active".to_string(),
        "children".to_string(),
        "class".to_string(),
        "icon".to_string(),
        "onclick".to_string(),
      ]
    }
  ]
}

pub fn get_props() -> Vec<PropRow> {
  vec![
    PropRow {
      name: "action".to_string(),
      r#type: "Html".to_string(),
      description: "Menu item action".to_string(),
      default: None
    },
    PropRow {
      name: "active".to_string(),
      r#type: "bool".to_string(),
      description: "Menu item active state".to_string(),
      default: Some("false".to_string())
    },
    PropRow {
      name: "children".to_string(),
      r#type: "Html".to_string(),
      description: "Component children".to_string(),
      default: None
    },
    PropRow {
      name: "class".to_string(),
      r#type: "Classes".to_string(),
      description: "Component custom class".to_string(),
      default: None
    },
    PropRow {
      name: "default_open".to_string(),
      r#type: "bool".to_string(),
      description: "Sidebar default open state".to_string(),
      default: Some("true".to_string())
    },
    PropRow {
      name: "icon".to_string(),
      r#type: "IconData".to_string(),
      description: "Sidebar trigger icon".to_string(),
      default: Some("IconData::LUCIDE_SIDEBAR".to_string())
    },
    PropRow {
      name: "onclick".to_string(),
      r#type: "Callback<()>".to_string(),
      description: "Menu item or Sidebar trigger onclick callback".to_string(),
      default: None
    },
    PropRow {
      name: "onopenchange".to_string(),
      r#type: "Option<Callback<bool>>".to_string(),
      description: "Sidebar onopenchange callback".to_string(),
      default: None
    },
    PropRow {
      name: "open".to_string(),
      r#type: "Option<bool>".to_string(),
      description: "Sidebar open state".to_string(),
      default: None
    },
    PropRow {
      name: "position".to_string(),
      r#type: "SidebarPosition".to_string(),
      description: "Left, Right".to_string(),
      default: Some("SidebarPosition::Left".to_string())
    },
    PropRow {
      name: "size".to_string(),
      r#type: "Size".to_string(),
      description: "Sidebar trigger button size".to_string(),
      default: Some("Size::Small".to_string())
    },
    PropRow {
      name: "style".to_string(),
      r#type: "String".to_string(),
      description: "Sidebar custom style".to_string(),
      default: None
    }
  ]
}