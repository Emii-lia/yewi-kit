use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum DocsRoute {
  #[at("/docs")]
  Docs,
  #[at("/docs/installation")]
  Installation,
  #[at("/docs/avatar")]
  AvatarPage,
  #[at("/docs/avatar-group")]
  AvatarGroupPage,
  #[at("/docs/badge")]
  BadgePage,
  #[at("/docs/button")]
  ButtonPage,
  #[at("/docs/card")]
  CardPage,
  #[at("/docs/carousel")]
  CarouselPage,
  #[at("/docs/checkbox")]
  CheckboxPage,
  #[at("/docs/collapse")]
  CollapsePage,
  #[at("/docs/divider")]
  DividerPage,
  #[at("/docs/dropdown")]
  DropdownPage,
  #[at("/docs/file-input")]
  FileInput,
  #[at("/docs/input")]
  InputPage,
  #[at("/docs/modal")]
  ModalPage,
  #[at("/docs/password-input")]
  PasswordInputPage,
  #[at("/docs/progress")]
  ProgressPage,
  #[at("/docs/quick-start")]
  QuickStartPage,
  #[at("/docs/radio")]
  RadioPage,
  #[at("/docs/select")]
  SelectPage,
  #[at("/docs/table")]
  TablePage,
  #[at("/docs/tabs")]
  TabsPage,
  #[at("/docs/textarea")]
  TextareaPage,
  #[at("/docs/toast")]
  ToastPage,
  #[not_found]
  #[at("/docs/404")]
  NotFound,
}
impl DocsRoute {
  pub fn iter() -> impl Iterator<Item = DocsRoute> {
    vec![
      DocsRoute::Installation,
      DocsRoute::AvatarPage,
      DocsRoute::AvatarGroupPage,
      DocsRoute::BadgePage,
      DocsRoute::ButtonPage,
      DocsRoute::CardPage,
      DocsRoute::CarouselPage,
      DocsRoute::CheckboxPage,
      DocsRoute::CollapsePage,
      DocsRoute::DividerPage,
      DocsRoute::DropdownPage,
      DocsRoute::FileInput,
      DocsRoute::InputPage,
      DocsRoute::ModalPage,
      DocsRoute::PasswordInputPage,
      DocsRoute::ProgressPage,
      DocsRoute::QuickStartPage,
      DocsRoute::RadioPage,
      DocsRoute::SelectPage,
      DocsRoute::TablePage,
      DocsRoute::TabsPage,
      DocsRoute::TextareaPage,
      DocsRoute::ToastPage,
      DocsRoute::NotFound
    ].into_iter()
  }

  pub fn to_string(&self) -> String {
    match self {
      DocsRoute::Docs => "Documentation".to_string(),
      DocsRoute::Installation => "Installation".to_string(),
      DocsRoute::AvatarPage => "Avatar".to_string(),
      DocsRoute::AvatarGroupPage => "Avatar Group".to_string(),
      DocsRoute::BadgePage => "Badge".to_string(),
      DocsRoute::ButtonPage => "Button".to_string(),
      DocsRoute::CardPage => "Card".to_string(),
      DocsRoute::CarouselPage => "Carousel".to_string(),
      DocsRoute::CheckboxPage => "Checkbox".to_string(),
      DocsRoute::CollapsePage => "Collapse".to_string(),
      DocsRoute::DividerPage => "Divider".to_string(),
      DocsRoute::DropdownPage => "Dropdown".to_string(),
      DocsRoute::FileInput => "File Input".to_string(),
      DocsRoute::InputPage => "Input".to_string(),
      DocsRoute::PasswordInputPage => "Password Input".to_string(),
      DocsRoute::ProgressPage => "Progress".to_string(),
      DocsRoute::QuickStartPage => "Quick Start".to_string(),
      DocsRoute::ModalPage => "Modal".to_string(),
      DocsRoute::RadioPage => "Radio".to_string(),
      DocsRoute::SelectPage => "Select".to_string(),
      DocsRoute::TablePage => "Table".to_string(),
      DocsRoute::TabsPage => "Tabs".to_string(),
      DocsRoute::TextareaPage => "Textarea".to_string(),
      DocsRoute::ToastPage => "Toast".to_string(),
      DocsRoute::NotFound => "Not Found".to_string(),
    }
  }
}