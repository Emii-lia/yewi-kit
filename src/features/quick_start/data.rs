use crate::app::docs::routes::DocsRoute;

#[derive(Clone)]
pub struct CardData {
  pub title: String,
  pub description: String,
  pub to: DocsRoute
}

pub fn get_common_components() -> Vec<CardData> {
  vec![
    CardData {
      title: "Button".to_string(),
      description: "Interactive clickable element with variants and sizes.".to_string(),
      to: DocsRoute::ButtonPage,
    },
    CardData {
      title: "Input".to_string(),
      description: "Form input field with support for various input types.".to_string(),
      to: DocsRoute::InputPage,
    },
    CardData {
      title: "Card".to_string(),
      description: "Container component for grouping content with consistent styling.".to_string(),
      to: DocsRoute::CardPage,
    },
    CardData {
      title: "Modal".to_string(),
      description: "Overlay component for displaying content in a modal window.".to_string(),
      to: DocsRoute::ModalPage,
    },
    CardData {
      title: "Avatar".to_string(),
      description: "Display user profile pictures with fallback support.".to_string(),
      to: DocsRoute::AvatarPage,
    },
    CardData {
      title: "Badge".to_string(),
      description: "Label component for tags, statuses, and categories.".to_string(),
      to: DocsRoute::BadgePage,
    }
  ]
}