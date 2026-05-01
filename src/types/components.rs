use yew_icons::IconData;
use crate::app::docs::routes::DocsRoute;

#[derive(Clone)]
pub struct ComponentNav {
  pub route: DocsRoute,
  pub icon: Option<IconData>
}

impl ComponentNav {
  pub fn new(route: DocsRoute, icon: Option<IconData>) -> Self {
    Self { route, icon }
  }
  pub fn group_components() -> Vec<(String, Vec<Self>)> {
    vec![
      ("Actions".to_string(), vec![
        ComponentNav::new(DocsRoute::ButtonPage, Some(IconData::LUCIDE_MOUSE_POINTER_CLICK)),
        ComponentNav::new(DocsRoute::DropdownPage, Some(IconData::LUCIDE_CHEVRONS_DOWN)),
        ComponentNav::new(DocsRoute::ModalPage, Some(IconData::LUCIDE_STICKY_NOTE)),
        ComponentNav::new(DocsRoute::ToastPage, Some(IconData::LUCIDE_BELL)),
      ]),
      ("Data Display".to_string(), vec![
        ComponentNav::new(DocsRoute::AvatarPage, Some(IconData::LUCIDE_USER)),
        ComponentNav::new(DocsRoute::AvatarGroupPage, Some(IconData::LUCIDE_USERS)),
        ComponentNav::new(DocsRoute::BadgePage, Some(IconData::LUCIDE_TAG)),
        ComponentNav::new(DocsRoute::CardPage, Some(IconData::LUCIDE_RECTANGLE_HORIZONTAL)),
        ComponentNav::new(DocsRoute::CarouselPage, Some(IconData::LUCIDE_IMAGE)),
        ComponentNav::new(DocsRoute::CollapsePage, Some(IconData::LUCIDE_CHEVRONS_DOWN_UP)),
        ComponentNav::new(DocsRoute::ProgressPage, Some(IconData::LUCIDE_BAR_CHART_HORIZONTAL)),
        ComponentNav::new(DocsRoute::TablePage, Some(IconData::LUCIDE_TABLE_2)),
      ]),
      ("Form".to_string(), vec![
        ComponentNav::new(DocsRoute::CheckboxPage, Some(IconData::LUCIDE_CHECK_SQUARE)),
        ComponentNav::new(DocsRoute::FileInput, Some(IconData::LUCIDE_UPLOAD)),
        ComponentNav::new(DocsRoute::FilePreview, Some(IconData::LUCIDE_FILES)),
        ComponentNav::new(DocsRoute::InputPage, Some(IconData::LUCIDE_FORM_INPUT)),
        ComponentNav::new(DocsRoute::PasswordInputPage, Some(IconData::LUCIDE_KEY)),
        ComponentNav::new(DocsRoute::RadioPage, Some(IconData::LUCIDE_CIRCLE_DOT)),
        ComponentNav::new(DocsRoute::SelectPage, Some(IconData::LUCIDE_LIST)),
        ComponentNav::new(DocsRoute::TextareaPage, Some(IconData::LUCIDE_TEXT_CURSOR_INPUT)),
      ]),
      ("Navigation".to_string(), vec![
        ComponentNav::new(DocsRoute::BreadcrumbsPage, Some(IconData::LUCIDE_CHEVRON_RIGHT)),
        ComponentNav::new(DocsRoute::DividerPage, Some(IconData::LUCIDE_SEPARATOR_VERTICAL)),
        ComponentNav::new(DocsRoute::LinkPage, Some(IconData::LUCIDE_LINK_2)),
        ComponentNav::new(DocsRoute::PaginationPage, Some(IconData::LUCIDE_CHEVRONS_LEFT_RIGHT)),
        ComponentNav::new(DocsRoute::SidebarPage, Some(IconData::LUCIDE_SIDEBAR)),
        ComponentNav::new(DocsRoute::TabsPage, Some(IconData::LUCIDE_LAYERS)),
      ])
    ]
  }
}