#[derive(Clone, Debug)]
pub struct WhySectionData {
  pub title: String,
  pub description: String,
}

pub fn get_why_data() -> Vec<WhySectionData> {
  vec![
    WhySectionData {
      title: "Add What You Need".to_string(),
      description: "Yew + Tailwind CSS + best practices, all set up out of the box. Start building immediately, no configuration needed.".to_string(),
    },
    WhySectionData {
      title: "Pre-configured Stack".to_string(),
      description: "Yew + Tailwind CSS + best practices, all set up out of the box. Start building immediately, no configuration needed.".to_string()
    },
    WhySectionData {
      title: "Component Ownership".to_string(),
      description: "You own the code. Components live in your project, fully customizable and extendable without vendor lock-in.".to_string(),
    },
    WhySectionData {
      title: "Easy Customization".to_string(),
      description: "Modify colors, spacing, and behavior directly in your codebase. No hidden internals, just clean Rust and CSS.".to_string(),
    },
    WhySectionData {
      title: "Built-in Themes".to_string(),
      description: "Start with a polished theme of your choice that you can easily customize or replace. Your app, your style.".to_string(),
    },
    WhySectionData {
      title: "Clean Architecture".to_string(),
      description: "Yewi-kit follows component-driven architecture, making it easy to build robust, scalable applications.".to_string(),
    }
  ]
}