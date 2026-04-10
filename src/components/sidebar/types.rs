#[derive(Clone, Debug, PartialEq)]
pub enum SidebarState {
  Collapsed,
  Expanded,
}

impl SidebarState {
  pub fn as_str(&self) -> &'static str {
    match self {
      SidebarState::Collapsed => { "collapsed" }
      SidebarState::Expanded => {"expanded" }
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SidebarPosition {
  Left,
  Right,
}

impl SidebarPosition {
  pub fn as_str(&self) -> &'static str {
    match self {
      SidebarPosition::Left => { "left" }
      SidebarPosition::Right => {"right" }
    }
  }

  pub fn prefix(&self, prefix: &str) -> String {
    format!(
      "{}-{}",
      prefix,
      self.as_str()
    )
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SidebarConfig {
  pub width: String,
  pub width_mobile: String,
  pub width_icon: String,
}

impl SidebarConfig {
  pub fn new(width: String, width_mobile: String, width_icon: String) -> Self {
    Self {
      width,
      width_mobile,
      width_icon,
    }
  }

  pub fn default() -> Self {
    Self::new("16rem".to_string(), "100%".to_string(), "2.5rem".to_string())
  }
}